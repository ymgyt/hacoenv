#!/bin/sh

set -u # treat unset variables as an error when performing parameter expansion.

# hacoenv installer script.

HACOENV_ROOT="${HACOENV_ROOT:-http://hacoenv.ymgyt.io}"
HACOENV_PATH="${HACOENV_PATH:-/usr/local/bin/hacoenv}"

usage() {
  cat 1>&2 <<EOF
hacoenv 0.1.0
The installer for hacoenv

USAGE:
  install.sh [FLAGS] [OPTIONS]

FLAGS:
  -y, --yes     Disable confirmation prompt.
  -h, --help    Prints help information.
OPTIONS:

EOF
}

main() {
  need_cmd mktemp
  need_cmd mkdir
  need_cmd chmod

  get_architecture
  local _arch="$RETVAL"

  # for future windows support
  local _ext=""
  case "$_arch" in
    *windows*)
      _ext=".ext"
      ;;
  esac

  local _url="${HACOENV_ROOT}/dist/${_arch}/hacoenv${_ext}"

  local _dir="$(mktemp --directory 2>/dev/null)"
  assert_nz "$_dir" "mktemp"

  local _file="${_dir}/hacoenv${_ext}"

  local _ansi_escapes_are_valid=false
  if [ -t 2 ]; then
    if [ "${TERM+set}" = 'set' ]; then
      case "$TERM" in
        xterm*|rxvt*|urxvt*|linux*|vt*)
          _ansi_escapes_are_valid=true
          ;;
      esac
    fi
  fi

  local need_tty=yes
  for arg in "$@"; do
    case "$arg" in
      -h|--help)
        usage
        exit 0
        ;;
      -y|--yes)
        need_tty=no
        ;;
      *)
        ;;
    esac
  done

  info "$_ansi_escapes_are_valid" 'downloading hacoenv'

  # download hacoenv then make executable
  ensure mkdir -p "$_dir"
  ensure downloader "$_url" "$_file" "$_arch"
  ensure chmod u+x "$_file"

  if [ ! -x "$_file" ]; then
    printf '%s\n' "cannot execute $_file" 1>&2
    exit 1
  fi

  if [ "$need_tty" = "yes" ]; then
    # this script was piped into shell, so doesn't have stdin to pass its children.
    if [ ! -t 1 ]; then
      err "unable to run interactively. run with -y to accept defaults."
    fi
  fi

  mv "$_file" "${HACOENV_PATH}"

  info "$_ansi_escapes_are_valid" "successfully installed to ${HACOENV_PATH}"
}

info() {
  if $1; then
    printf '\33[1minfo:\33[0m %s\n' "$2" 1>&2
  else
    printf 'info: %s\n' "$2" 1>&2
  fi
}

get_architecture() {
  local _ostype
  _ostype="$(uname -s)"  # --kernel-name
  _cputype="$(uname -m)" # --machine

  case "$_ostype" in

    Darwin)
      _ostype=apple-darwin
      ;;
    *)
        err "unrecognized/unsupported OS type: $_ostype"
        ;;

  esac

  case "$_cputype" in

    x86_64 | x86-64 | x64 | amd64)
      _cputype=x86_64
      ;;

    *)
      err "unknown/unsupported CPU type: $_cputype"
      ;;
  esac

  _arch="${_cputype}-${_ostype}"

  RETVAL="$_arch"
}

downloader() {
  local _di
  local _ciphersuites
  local _err
  local _status
  local _url="$1"
  local _out="$2"
  local _arch="$3"

  if check_cmd curl; then
    _did=curl
  elif check_cmd wget; then
    _did=wget
  else
    _did='curl or wget' # for error message.
  fi

  if [ "$1" = --check ]; then
    need_cmd "$_did"
  elif [ "$_did" = curl ]; then
    _err=$(curl --silent --show-error --fail --location "$_url" --output "$_out" 2>&1)
    _status=$?

    if [ -n "$_err" ]; then
      echo "$_err" >&2
      if echo "$_err" | grep -q ' 404 Not found$'; then
        err "hacoenv for platform '$_arch' not found"
      fi
    fi
    return $_status
  elif [ "$_did" = wget ]; then
    err "wget currently unsupported"
  else
    err "unexpected downloader" # should not reach here
  fi
}

ignore() {
  "$@"
}

say() {
  printf 'hacoenv: %s\n' "$1"
}

err() {
  say "$1" >&2
  exit 1
}
need_cmd() {
  if ! check_cmd "$1"; then
    err "need '$1' (command not found)"
  fi
}

check_cmd() {
  command -v "$1" > /dev/null 2>&1
}

assert_nz(){
  if [ -z "$1" ]; then err "assert_nz $2"; fi
}

ensure() {
  if ! "$@"; then err "command failed: $*"; fi
}


main "$@" || exit 1
