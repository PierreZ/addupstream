# AddUpstream [![Build Status](https://travis-ci.org/PierreZ/addupstream.svg?branch=master)](https://travis-ci.org/PierreZ/addupstream)

A small cli to automatically add upstream remotes to a git project. Only working with Github for the moment.

[![asciicast](https://asciinema.org/a/123468.png)](https://asciinema.org/a/123468)

## How to build

```bash
git clone https://github.com/PierreZ/addupstream
cd upstream
cargo build
```

## How to install

```bash
cargo install --git https://github.com/PierreZ/addupstream
```

## How to use

just run the binary in a git repo that you forked using Github' UI.

```bash
addupstream
```

## Options

```bash
addupstream --help
```
