
# kubectl-copi

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/worldline/kubectl-copi/main.yml)
![GitHub Release](https://img.shields.io/github/v/release/worldline/kubectl-copi)
![GitHub License](https://img.shields.io/github/license/worldline/kubectl-copi)

A [kubectl](https://github.com/kubernetes/kubectl) extension to pick configuration options - `copi` for configuration PIcker.

## Installation

### With Krew

```
kubectl krew install --manifest-url https://raw.githubusercontent.com/worldline/kubectl-copi/main/krew-manifest.yaml
```

### Manually

Get the appropriate binary from the latest release, and put it somewhere in your `PATH` so that `kubectl` can find it. More information on the [Extend kubectl with plugin](https://kubernetes.io/docs/tasks/extend-kubectl/kubectl-plugins/) page.

## Usage

Once installed, it can be invoked using `kubectl copi`:

```
Usage: kubectl-copi [COMMAND]

Commands:
  ctx   Pick a context from the kube config
  ns    Pick a namespace for the current context - Default command if none is specified
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Prerequisites

For some of the commands, you need to be logged in on the target cluster; this pluggin will use the current context in your `~/.kube/config` file.

## Building

This project aims at staying small and simple; for the moment the build process is `cargo build`.

[`cargo-watch`](https://github.com/watchexec/cargo-watch) is used for local development; use `make pipeline` to use it.

## Contributing

All contributions are welcome, open an issue if you face any problem; open a PR if you want to make an improvement of spontaneously fix something.

## TODO List

- Generate Krew manifest along with the release
- Windows & Mac support

