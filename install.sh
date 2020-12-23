#!/bin/sh

# hacoenv installer script.

main() {
  get_ostype
  local _ostype="$RETVAL"

  say $_ostype
}

get_ostype() {
  local _ostype
  _ostype="$(uname -s)"

  case "$_ostype" in
    Darwin)
      _ostype=apple-darwin
      ;;
    *)
        err "unrecognized/unsupported OS type: $_ostype"
        ;;
  esac

  RETVAL="$_ostype"
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


main "$@" || exit 1
