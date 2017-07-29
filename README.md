# ![rubigo-coreutils](logo.png)
[![docs: published](https://img.shields.io/badge/docs-published-green.svg)](https://rubigo.github.io/coreutils/doc/rubigo_pwd) 
[![current tag](https://img.shields.io/github/tag/rubigo/pwd.svg)](CHANGELOG.md) 
[![travis build status](https://travis-ci.org/rubigo/pwd.svg?branch=master)](https://travis-ci.org/rubigo/pwd)
[![codecov](https://codecov.io/gh/rubigo/pwd/branch/master/graph/badge.svg)](https://codecov.io/gh/rubigo/pwd)

This the `pwd` utility, part of
[rubigo-coreutils](https://github.com/rubigo/coreutils), which prints the
current working directory to `stdout`.

## Features

It gathers the current working directory from the environment variables. The
function to access it is exposed in the library. It is feature-compatible with
GNU `pwd` and OpenBSD `pwd`.

## Dependencies

name | current version | description
---: | :-------------: | :----------
[clap](https://github.com/kbknapp/clap-rs) | [![Crates.io](https://img.shields.io/crates/v/clap.svg)](https://crates.io/crates/clap) | *Used to parse command-line arguments*

# Todo

- [ ] test everything
