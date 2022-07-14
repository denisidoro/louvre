# Louvre

A lightweight, cross-platform game scraper written in Rust.

### Example

Running the app using [Termux](https://termux.com) on Android:
![Termux](https://user-images.githubusercontent.com/3226564/178999663-df84eb27-dec1-40ae-8e1d-7705988ed86d.jpeg)

Viewing results in [Pegasus](https://pegasus-frontend.org) on Android:
![Pegasus](https://user-images.githubusercontent.com/3226564/178999677-df2aa47e-51e1-4935-8cff-4700efd5b364.jpeg)

### Installation

Download the correct binary in the [releases page](https://github.com/denisidoro/louvre/releases) and place it somewhere in your `$PATH`.

### Usage

1. `louvre config init`
1. edit the generated file accordingly
   - this also includes setting your [IGDB client credentials](https://www.igdb.com/api)
1. `louvre scrape`
1. `louvre media download`
1. `louvre pegasus gen`

### Status

It works, assuming: 
- you know how to use the terminal in your device your choice
- you can get IGDB client credentials 

### Roadmap

- drop the requirement for setting IGDB client credentials
- support [TGDB](https://thegamesdb.net), besides IGDB
- support more frontends, besides Pegasus

### Etymology

Game collection > collection > museum > [Louvre](https://en.wikipedia.org/wiki/Louvre)