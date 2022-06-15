# OpenBabel bindings for Rust

This repository is composed of two packages:
- openbabel-sys: FFI to OpenBabel C++ codes.
- openbabel: Rust wrappers for easy usage.

***multi-thread mode will encounter memory issue and unexpected exit from time to time***

## Binding Progress

:white_check_mark: OBConversion

:white_check_mark: OBMol 

:white_check_mark: OBFingerprint 

:white_check_mark: OBSmartsPatten 


## References
- Code structure: [libcurl bindings for Rust](https://github.com/alexcrichton/curl-rust)

### OpenBabel 
- Openbabel [documentation](http://openbabel.org/wiki/Main_Page)
- [Substructure Searching](http://openbabel.org/dev-api/group__substructure.shtml)
- [OBSmartsPattern Class Reference](http://openbabel.org/dev-api/classOpenBabel_1_1OBSmartsPattern.shtml)