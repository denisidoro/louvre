extern crate louvre;

use httpmock::prelude::*;
use louvre::prelude::*;
use std::env;
use std::fs;
use walkdir::WalkDir;

struct Runner {
    server: MockServer,
    config_path: String,
    fixtures_dir: PathBuf,
    testdata_dir: PathBuf,
}

#[test]
fn test_e2e() {
    let (fixtures_dir, testdata_dir) = copy_roms().expect("copy_roms failed");
    set_home_dir(&testdata_dir).expect("set_home_dir failed");
    let server = MockServer::start();
    let config_path = set_config(&server, &testdata_dir).expect("set_config failed");

    let runner = Runner {
        server,
        config_path,
        fixtures_dir,
        testdata_dir,
    };

    runner.test_config().expect("test_config failed");
    runner.test_scrape().expect("test_scrape failed");
    runner.test_media().expect("test_media failed");
    runner.test_pegasus().expect("test_pegasus failed");
}

fn copy_roms() -> Result<(PathBuf, PathBuf)> {
    let pwd = env::current_dir()?;
    let project_dir = pwd.parent().context("no parent")?;

    let fixtures_dir = project_dir.join("fixtures");
    let roms_dir = fixtures_dir.join("ROMs");

    let testdata_dir = {
        let mut p = project_dir.to_owned();
        p.push("target");
        p.push("testdata");
        p
    };

    let _ = fs::remove_dir_all(&testdata_dir);

    let entries = WalkDir::new(&roms_dir)
        .max_depth(6)
        .into_iter()
        .filter_map(|e| e.ok());

    let testdata_roms_dir = testdata_dir.join("ROMs");

    for entry in entries {
        if entry.metadata()?.is_file() {
            let new_path = {
                let relative = entry.path().strip_prefix(&roms_dir)?;
                let mut p = testdata_roms_dir.to_owned();
                p.push(relative);
                p
            };
            let new_parent = new_path.parent().context("no parent")?;
            fs::create_dir_all(&new_parent)?;
            fs::copy(entry.path(), new_path)?;
        }
    }

    Ok((fixtures_dir, testdata_dir))
}

fn set_home_dir(testdata_dir: &Path) -> Result<()> {
    let home_dir = testdata_dir.join("home");
    fs::create_dir_all(&home_dir)?;
    env::set_var("HOME", &home_dir.to_string());
    Ok(())
}

fn set_config(server: &MockServer, testdata_dir: &Path) -> Result<String> {
    let base_url = &server.base_url();

    let config = format!(
        r#"
parallelism:
  workers: 1
twitch:
  client_id: test_client_id
  client_secret: test_client_secret
  id_base_url: {base_url}
igdb:
  api_base_url: {base_url}
  images_base_url: {base_url}
  denylisted_name_substrings:
    - bad game
collections:
  - name: Game Boy Advance
    path: {testdata}/ROMs/gba
    extensions: 
      - zip
    platform: gba
    denylist:
      - Link to the Past
    launch: retroarch --gba ${{file}}
  - name: Nintendo 3DS
    path: {testdata}/ROMs/3ds
    extensions: 
      - cia
      - 3ds
    platform: 3ds
    denylist:
      - Update
      - Pok[ée]
    launch: retroarch --3ds ${{file}}
  - name: Playstation 3
    path: {testdata}/ROMs/ps3
    extensions: 
      - bin
    platform: ps3
    denylist: ~
    launch: retroarch --ps3 ${{file}}
tracing:
  time: false
  level: {bin}=trace
"#,
        bin = PROJECT_NAME,
        base_url = base_url,
        testdata = testdata_dir.to_string()
    );

    let config_path = testdata_dir.join("config.yaml");
    fs::write(&config_path, config.trim())?;

    Ok(config_path.to_string())
}

impl Runner {
    fn call(&self, extra_args: &[&str]) -> Result<()> {
        let mut extra_args_vec: Vec<String> = extra_args
            .to_owned()
            .iter_mut()
            .map(|s| s.to_owned())
            .collect();

        let mut args = vec![
            PROJECT_NAME.to_owned(),
            "--config".to_owned(),
            self.config_path.clone(),
        ];

        args.append(&mut extra_args_vec);

        louvre::boot(Some(args.iter().map(|x| x.as_str()).collect()))
    }

    fn test_config(&self) -> Result<()> {
        self.call(&["config", "init"])?;

        Ok(())
    }

    fn test_scrape(&self) -> Result<()> {
        let json = include_str!("../../fixtures/responses/metadata.json");

        let auth = r#"{
            "access_token": "test_access_token", 
            "expires_in": 48853900,
            "token_type": "bearer"
         }"#;

        let mut twitch_mock = self.server.mock(|when, then| {
            when.method(POST).path_contains("oauth");
            then.status(200)
                .header("content-type", "application/json")
                .body(auth);
        });

        let mut igdb_mock = self.server.mock(|when, then| {
            when.method(POST).path_contains("games");
            then.status(200)
                .header("content-type", "application/json")
                .body(json);
        });

        self.call(&["scrape"])?;

        twitch_mock.assert_hits(1);
        igdb_mock.assert_hits(12);

        twitch_mock.delete();
        igdb_mock.delete();

        let meta = self.testdata_dir.join(PathBuf::from_iter(&[
            "ROMs",
            "gba",
            "meta",
            "The Legend of Zelda The Minish Cap",
            "meta.yaml",
        ]));
        assert!(meta.exists());

        Ok(())
    }

    fn test_media(&self) -> Result<()> {
        let image = {
            let mut p = self.fixtures_dir.to_owned();
            p.push("images");
            p.push("green.png");
            p
        };

        let mut mock = self.server.mock(|when, then| {
            when.method(GET);
            then.status(200)
                .header("content-type", "image/png")
                .body_from_file(image.to_string());
        });

        self.call(&["media", "download"])?;

        mock.assert_hits(36);
        mock.delete();

        let zelda = self.testdata_dir.join(PathBuf::from_iter(&[
            "ROMs",
            "gba",
            "meta",
            "The Legend of Zelda The Minish Cap",
        ]));
        assert!(zelda.join("background.jpg").exists());
        assert!(zelda.join("boxFront.jpg").exists());
        assert!(zelda.join("screenshot.jpg").exists());

        Ok(())
    }

    fn test_pegasus(&self) -> Result<()> {
        self.call(&["pegasus", "gen"])?;

        self.assert_meta(
            "gba",
            &[
                "game: __NAME__",
                "file: Fullmetal Alchemist Stray Rondo.zip",
                "release: 2004-11-04",
                "genres: Role-playing (RPG), Adventure",
                "assets.boxFront: meta/Fullmetal Alchemist Stray Rondo/boxFront.jpg",
                "assets.background: meta/Fullmetal Alchemist Stray Rondo/background.jpg",
                "assets.screenshot: meta/Fullmetal Alchemist Stray Rondo/screenshot.jpg",
                "rating: 90%",
                "developers: Capcom, Flagship",
                "publishers: Nintendo",
                "players: 2",
                "summary: This game is super fun",
                "description: Lorem ipsum",
            ],
        )?;

        self.assert_meta(
            "ps3",
            &[
                "file: BCES01175-[Uncharted 3 Drakes Deception]/PS3_GAME/USRDIR/EBOOT.BIN",
                "assets.boxFront: meta/Uncharted 3 Drakes Deception/boxFront.jpg",
            ],
        )?;

        Ok(())
    }

    fn assert_meta(&self, platform: &str, substrs: &[&str]) -> Result<()> {
        let meta_path = self.testdata_dir.join(PathBuf::from_iter(&[
            "ROMs",
            platform,
            "metadata.pegasus.txt",
        ]));

        let meta = fs::read_to_string(meta_path)?;

        for substr in substrs {
            assert!(meta.contains(substr));
        }

        Ok(())
    }
}
