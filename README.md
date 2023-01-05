# OpenBabel bindings for Rust

[![Latest Version](https://img.shields.io/crates/v/openbabel.svg)](https://crates.io/crates/openbabel)
[![Documentation](https://docs.rs/openbabel/badge.svg)](https://docs.rs/openbabel)
[![License](https://img.shields.io/github/license/rogerwq/openbabel-rust.svg)](LICENSE)
[![Build](https://github.com/rogerwq/openbabel-rust/workflows/CI/badge.svg)](https://github.com/rogerwq/openbabel-rust/actions)

This repository is composed of two packages:
- openbabel-sys: FFI to OpenBabel C++ codes.
- openbabel: Rust wrappers for easy usage.

*** Attentions ***
- multi-thread mode will encounter memory issue and unexpected exit from time to time***
- Currently not working under Windows***
- Low-version C++ compiler could lead to unexpected compiling warnings***

## Wrapping Progress

:white_check_mark: OBConversion

:white_check_mark: OBMol 

:white_check_mark: OBFingerprint 

:white_check_mark: OBSmartsPatten 

:white_check_mark: OBForceField

## Logs
### v0.5
- v0.5.2: smartsparttern.rs: add a test case of symmetric query
- v0.5.1: smartspattern.rs: find_match() return ```Vec<Vec<...>>```

## References
- Code structure: [libcurl bindings for Rust](https://github.com/alexcrichton/curl-rust)
- Openbabel [documentation](http://openbabel.org/wiki/Main_Page)
- [Substructure Searching](http://openbabel.org/dev-api/group__substructure.shtml)
- [OBSmartsPattern Class Reference](http://openbabel.org/dev-api/classOpenBabel_1_1OBSmartsPattern.shtml)
