# hacoenv

[Hacobu](https://movo.co.jp/) local development setup tool.

## Install

```console
curl -sSf http://hacoenv.ymgyt.io | sh
```

## Usage

### Setup product environment

```console
# init fleet environment
$ hacoenv fleet init
```

## Environment variables

* `HACOENV_LOG` : RUST logging directive. (`info`, `hacoenv=debug`, ...)
* `HACOENV_LOGO`: print hacobu logo. (`true` | `false`)


## TODO

- [ ] detect latest release.
