fn main() {
    let version = "3.1.1";
    let target = std::env::var("TARGET").unwrap();
    let windows = target.contains("windows");
    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    if !include.exists() {
        std::fs::create_dir(&include).unwrap();
    }

    if !std::path::Path::new("openbabel/.git").exists() {
        let _ = std::process::Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status().unwrap();
    }

    // header files for data
    if windows {
        return println!("windows not supported yet");
    } else {
        match std::process::Command::new("sh")
        .args(&["scripts/build.sh"])
        .status() {
            Ok(_) => {
                for header in [
                    "atomtyp.h",
                    "bondtyp.h",
                    "phmodeldata.h",
                    "resdata.h",
                    "torlib.h",
                    "types.h",
                    "atomizationenergies.h",
                    "spacegroups.h",
                    "ringtyp.h"
                ].iter()
                {
                    std::fs::rename(
                        format!("openbabel/data/{}", header),
                        include.join(header)
                    ).unwrap();
                }
            },
            Err(e) => {
                return println!("Generate data header files error: {:?}", e);
            }
        }
    }

    // Copy data directory
    let data_ob = dst.join("data").join(&version);
    if !data_ob.exists() {
        std::fs::create_dir_all(&data_ob).unwrap();
        fs_extra::dir::copy("openbabel/data", data_ob.to_str().unwrap(), &fs_extra::dir::CopyOptions::new()).unwrap();
    }

    // babelconfig.h
    let include_ob = include.join("openbabel");
    if !include_ob.exists() {
        std::fs::create_dir(include_ob).unwrap();
    }
    let contents = std::fs::read_to_string("openbabel/src/config.h.cmake").unwrap();
    std::fs::write(
        include.join("openbabel/babelconfig.h"),
        contents
            .replace("@BABEL_DATADIR@", data_ob.to_str().unwrap())
            .replace("@BABEL_VERSION@", &version)
            .replace("@MODULE_EXTENSION@", "na")
            .replace("@OB_MODULE_PATH@", "na") 
    ).unwrap();

    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("{:?}", dst);
}