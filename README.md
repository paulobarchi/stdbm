<!-- omit in TOC -->
# stdbm

> **Sentence-Tag DataBase Manager**

[![Crates.io](https://img.shields.io/crates/v/stdbm?style=flat-square)](https://crates.io/crates/stdbm)
[![Crates.io](https://img.shields.io/crates/d/stdbm?style=flat-square)](https://crates.io/crates/stdbm)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/paulobarchi/stdbm/LICENSE.md)
[![Build Status](https://img.shields.io/github/workflow/status/paulobarchi/stdbm/CI/style=flat-square)](https://github.com/paulobarchi/stdbm/actions/workflows/ci.yml)
[![Contributors](https://img.shields.io/github/contributors/paulobarchi/stdbm?style=flat-square)](https://github.com/paulobarchi/stdbm/graphs/contributors)

## About

Simple manager for databases with sentences and tags written in Rust.
* Sentence-tag relationship is 1:N;
* I/O only with csv files for now.

## Usage

```bash
stdbm 
Sentences-tags database manager

USAGE:
    stdbm <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       Add new register
    filter    Filter and display datatable registers which contain filter_string
    help      Print this message or the help of the given subcommand(s)
    list      List registers of datatable
    load      Load csv file to manipulate data
    rm        Remove register by id
    save      Save datatable to file path
    start     Start (load) DB with info from config file
    update    Update register value by id
```

The start subcommand uses the config file information to start up the database. The default config file can be found at [`config/default.toml`](config/default.toml) -- current content:

```toml
config_type = "toml"

[data_io]
data_file_type = "csv"
sentences_file = "sentences.csv"
tags_file = "tags.csv"
sentences_tags_file = "sentences-tags.csv"
check_ids_on_load = false
overwrite = true
bckp_folder = "bckp/"
```

## License

Licensed under [Apache 2.0](LICENSE.md).
