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

## Todo

- [ ] test everything

## Specification

Specification is adapted from the *The Open Group Base Specifications
Issue 7*, [section XCU
`pwd`](http://pubs.opengroup.org/onlinepubs/9699919799/nframe.html), to be as
compatible with the POSIX standard as possible (or as practical).

### Synopsis

    pwd [-L|-P]

### Description

The `pwd` utility shall write to standard output an absolute pathname of the
current working directory, which does not contain the filenames `‘.’` or 
`‘..’`.

### Options

flag | description
---: | :----------
`-L`, `--logical` | If the `PWD` environment variable contains an absolute pathname of the current directory and the pathname does not contain any components that are `‘.’` or `‘..’`, `pwd` shall write this pathname to standard output, except that if the `PWD` environment variable is longer than `PATH_MAX` bytes including the terminating null, it is unspecified whether pwd writes this pathname to standard output or behaves as if the `-P` option had been specified. Otherwise, the `-L` option shall behave as the `-P` option.
`-P`, `--physical` | The pathname written to standard output shall not contain any components that refer to files of type symbolic link. If there are multiple pathnames that the pwd utility could write to standard output, one beginning with a single slash character `‘/’` and one or more beginning with two slash characters `‘//’`, then it shall write the pathname beginning with a single slash character. The pathname shall not contain any unnecessary slash characters after the leading one or two slash characters. 

The POSIX standard only mandates that the short options `-L` and `-P` are
present, however for convenience we also allow the long versions `--logical` and
`--physical`.

In addition to this, the POSIX standard specifies that:

>   If both `-L` and `-P` are specified, the last one shall apply. If neither
>   `-L` nor `-P` is specified, the `pwd` utility shall behave as if `-L` had
>   been specified.

However, this behaviour is not followed. Instead, if neither `-L` not `-P` are
specified, the `pwd` utility behaves as if `-P` had been set. To enable the
POSIX-conformant behaviour, one can set the environment variable
`POSIXLY_CORRECT`. 

### Environment Variables

variable | usage
-------: | :----
`PWD` | An absolute pathname of the current working directory. If an application sets or unsets the value of `PWD`, the behavior of `pwd` is unspecified.

### Standard Output

The `pwd` utility output is an absolute pathname of the current working directory:

>   `“%s\n”`, *&lt;directory pathname&gt;*

### Exit Status

The following exit values shall be returned:

value | interpretation
----: | :-------------
`0` | Successful completion.
`>0` | An error occurred.
