# wup [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

A simple command line tool to manage version of other command line tools made for Windows.

This is just a fun side project to build something like [Scoop](https://scoop.sh), use this at your own risk!

# Installation

Download the latest release from [here](https://github.com/Yash-Garg/wup/actions/workflows/build.yml).

# Configuration

The configuration file is a simple YML file that contains the list of tools to manage, it should be located in the same directory as the executable and should be named [`config.yml`](https://github.com/Yash-Garg/wup/blob/develop/config.yml).

```yml
repos:
  - owner: sharkdp
    name: fd
  - owner: sharkdp
    name: bat
    force_tag: v0.22.1 # Optional
    ...
```

# Usage

The following is the output of `wup --help`:

```bash
❯ wup
A CLI tools version manager for Windows

Usage: wup.exe <COMMAND>

Commands:
  config   Prints the current config
  vstores  Prints the current version stores
  update   Updates all the tools in the current version store
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Validate Configuration

To validate the configuration file, simply run the following command:

```bash
wup config
```

It should show the list of [`RepoConfig`](https://github.com/Yash-Garg/wup/blob/develop/src/models/config.rs), which is the internal representation of the configuration file.

### Install or Update

To install or update the tools in the configuration file, simply run the following command:

```bash
wup update
```

### Version Store

To list out the current version store data, run the following command:

```bash
wup vstores
```

It will show the list of [`VersionStore`](https://github.com/Yash-Garg/wup/blob/develop/src/models/config.rs) which is the internal representation of the version store data.

# License

```
MIT License

Copyright (c) 2023 Yash Garg

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
