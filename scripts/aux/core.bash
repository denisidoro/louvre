#!/usr/bin/env bash
set -euo pipefail

if ${X_MODE:-false}; then
   set -x
fi


# =====================
# misc
# =====================

export ANDROID_HOST="192.168.0.12"
export ANDROID_PORT="8022"

export LOUVRE_BIN_NAME="louvre"


# =====================
# paths
# =====================

export CARGO_DEFAULT_BIN="${HOME}/.cargo/bin"
export BIN_DIR="${BIN_DIR:-"$CARGO_DEFAULT_BIN"}"


# =====================
# logging
# =====================

echoerr() {
   echo "$@" 1>&2
}

tap() {
   echoerr "$@"
   "$@"
}

log::ansi() {
   local bg=false
   case "$@" in
      *reset*) echo "\e[0m"; return 0 ;;
      *black*) color=30 ;;
      *red*) color=31 ;;
      *green*) color=32 ;;
      *yellow*) color=33 ;;
      *blue*) color=34 ;;
      *purple*) color=35 ;;
      *cyan*) color=36 ;;
      *white*) color=37 ;;
   esac
   case "$@" in
      *regular*) mod=0 ;;
      *bold*) mod=1 ;;
      *underline*) mod=4 ;;
   esac
   case "$@" in
      *background*) bg=true ;;
      *bg*) bg=true ;;
   esac

   if $bg; then
      echo "\e[${color}m"
   else
      echo "\e[${mod:-0};${color}m"
   fi
}

_log() {
   local template="$1"
   shift
   echoerr "$(printf "$template" "$@")"
}

_header() {
   local TOTAL_CHARS=60
   local total=$TOTAL_CHARS-2
   local size=${#1}
   local left=$((($total - $size) / 2))
   local right=$(($total - $size - $left))
   printf "%${left}s" '' | tr ' ' =
   printf " $1 "
   printf "%${right}s" '' | tr ' ' =
}

log::header() { _log "\n$(log::ansi bold)$(log::ansi purple)$(_header "$1")$(log::ansi reset)\n"; }
log::success() { _log "$(log::ansi green)✔ %s$(log::ansi reset)\n" "$@"; }
log::error() { _log "$(log::ansi red)✖ %s$(log::ansi reset)\n" "$@"; }
log::warning() { _log "$(log::ansi yellow)➜ %s$(log::ansi reset)\n" "$@"; }
log::note() { _log "$(log::ansi blue)%s$(log::ansi reset)\n" "$@"; }

header() {
   echoerr "$*"
   echoerr
}

die() {
   log::error "$@"
   exit 42
}


# =====================
# security
# =====================

sha256() {
   if command_exists sha256sum; then
      sha256sum
   elif command_exists shasum; then
      shasum -a 256
   elif command_exists openssl; then
      openssl dgst -sha256
   else
      log::note "Unable to calculate sha256!"
      exit 43
   fi
}


# =====================
# code
# =====================

version_from_toml() {
   cat "${LOUVRE_HOME}/Cargo.toml" \
      | grep version \
      | head -n1 \
      | awk '{print $NF}' \
      | tr -d '"' \
      | tr -d "'"
}


# =====================
# platform
# =====================

command_exists() {
   type "$1" &>/dev/null
}

get_target() {
   local -r unamea="$(uname -a)"
   local -r archi="$(uname -sm)"

   local target
   case "$unamea $archi" in
      *arwin*) target="x86_64-apple-darwin" ;;
      *inux*x86*) target="x86_64-unknown-linux-musl" ;;
      *ndroid*aarch*|*ndroid*arm*) target="aarch64-linux-android" ;;
      *inux*aarch*|*inux*arm*) target="armv7-unknown-linux-musleabihf" ;;
      *) target="" ;;
   esac

   echo "$target"
}

get_shell() {
   echo "$SHELL" | xargs basename
}