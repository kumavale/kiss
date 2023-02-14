# 😘 kiss

[![Actions Status](https://github.com/kumavale/kiss/workflows/CI/badge.svg)](https://github.com/kumavale/kiss/actions)
[![Crates.io](https://img.shields.io/crates/v/kiss.svg)](https://crates.io/crates/kiss)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

`kiss` is a `kill` clone.  
Sends a unix signal to the specified process id.  

## Install

```
$ cargo install kiss
```

## Usage

```
$ kiss --help
kiss: send a signal to a process

Usage: kiss <OPTIONS> [PID]...

Options:
  -s, --signal <SIGNAL>  Specify the signal
  -l, --list             List signal names
  -h, --help             Show help
  -v, --version          Show version
```

## e.g.

```
$ kiss --signal kill 12345
SIGKILL to 12345
```

```
$ kiss --signal term 1 2 3
SIGTERM to 1
SIGTERM to 2
SIGTERM to 3
```

```
$ kiss --signal kill 999999
failed: `ESRCH: No such process`
```

