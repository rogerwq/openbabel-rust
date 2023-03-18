# How openbabel source is being patched?

## The purposes
- the original codes created a global variable "obErrorLog" which lead to memory issues when multi-threading is applied.

## Directories
- directory "openbabel" is the original source
- directory "openbabel-extra" contains files which will replace the original corresponding files.
- directory "openbabel-patched" is auto-generated from build.rs. The auto-generation process only happens when this directory doesn't exist. Deleting this directory will make build.rs to re-generate this directory.

## The auto-generating process
- directory "include" and "src" will be copied from directory "openbabel" to "openbabel-patched"
- file content could be changed while copying according to replacements defined in build.rs.
- files from "openbabel-extra" will be copyied 