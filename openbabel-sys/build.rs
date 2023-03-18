use std::io::Write;

fn main() {
    let version = "3.1.1";
    // let target = std::env::var("TARGET").unwrap();
    // let windows = target.contains("windows");
    let dst = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    if !include.exists() {
        std::fs::create_dir(&include).unwrap();
    }

    // if !std::path::Path::new("openbabel/.git").exists() {
    //     match std::process::Command::new("git")
    //         .args(&["submodule", "update", "--init"])
    //         .status().unwrap();
    // }

    // copy data directory
    let data_ob = dst.join("data").join(&version);
    if !data_ob.exists() {
        std::fs::create_dir_all(&data_ob).unwrap();
        fs_extra::dir::copy("src/data", data_ob.to_str().unwrap(), &fs_extra::dir::CopyOptions::new()).unwrap();
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
            .replace("#cmakedefine HAVE_CONIO_H 1", "")
            .replace("#cmakedefine HAVE_SRANDDEV 1", "")
            .replace("#cmakedefine SCANDIR_NEEDS_CONST 1", "")
            .replace("#cmakedefine", "#define")
            .replace("@OB_SHARED_PTR_IMPLEMENTATION@", "std::shared_ptr")
            .replace("@OB_SHARED_PTR_HEADER@", "memory")
    ).unwrap();

    // patch 
    let dir_ob_patch = std::path::Path::new("openbabel-patch");
    if !dir_ob_patch.exists() {
        // copy all file in "include" & src
        std::fs::create_dir(&dir_ob_patch).unwrap();
        let dir_ob = std::path::Path::new("openbabel");
        fs_extra::dir::copy(dir_ob.join("include").to_str().unwrap(), dir_ob_patch.to_str().unwrap(), &fs_extra::dir::CopyOptions::new()).unwrap();
        fs_extra::dir::copy(dir_ob.join("src"), dir_ob_patch.to_str().unwrap(), &fs_extra::dir::CopyOptions::new()).unwrap();

        //  Patch by replacing
        for entry in walkdir::WalkDir::new(std::path::PathBuf::from("openbabel-patch")) {
            let entry = entry.unwrap();
            match entry.path().extension() {
                Some(ext) => {
                    if ext == "cpp" || ext == "h" {
                        let file_content = std::fs::read_to_string(entry.path()).unwrap();
                        let re = regex::Regex::new(r"OBMessageHandler obErrorLog").unwrap();
                        let after = re.replace_all(file_content.as_str(), "thread_local OBMessageHandler obErrorLog");
                        let mut file = std::fs::File::create(entry.path()).unwrap();
                        file.write(after.as_bytes()).unwrap();
                    }
                }
                None => ()
            }
        }

        //  Patch by overwriting 
        let dir_ob_extra = std::path::Path::new("openbabel-extra");
        let mut copy_option = fs_extra::dir::CopyOptions::new();
        copy_option.overwrite = true;
        fs_extra::dir::copy(dir_ob_extra.join("include").to_str().unwrap(), dir_ob_patch.to_str().unwrap(), &copy_option).unwrap();
    }



    // Compiling
    cxx_build::bridge("src/lib.rs")
        .file("openbabel-patch/src/base.cpp")
        .file("openbabel-patch/src/atom.cpp")
        .file("openbabel-patch/src/bond.cpp")
        .file("openbabel-patch/src/oberror.cpp")
        .file("openbabel-patch/src/tokenst.cpp")
        .file("openbabel-patch/src/generic.cpp")
        .file("openbabel-patch/src/rand.cpp")
        .file("openbabel-patch/src/graphsym.cpp")
        .file("openbabel-patch/src/ring.cpp")
        .file("openbabel-patch/src/phmodel.cpp")
        .file("openbabel-patch/src/obiter.cpp")
        .file("openbabel-patch/src/builder.cpp")
        .file("openbabel-patch/src/plugin.cpp")
        .file("openbabel-patch/src/data.cpp")
        .file("openbabel-patch/src/locale.cpp")
        .file("openbabel-patch/src/obutil.cpp")
        .file("openbabel-patch/src/descriptor.cpp")
        .file("openbabel-patch/src/elements.cpp")
        .file("openbabel-patch/src/typer.cpp")
        .file("openbabel-patch/src/chains.cpp")
        .file("openbabel-patch/src/bitvec.cpp")
        .file("openbabel-patch/src/parsmart.cpp")
        .file("openbabel-patch/src/residue.cpp")
        .file("openbabel-patch/src/mol.cpp")
        .file("openbabel-patch/src/transform.cpp")
        .file("openbabel-patch/src/obconversion.cpp")
        .file("openbabel-patch/src/format.cpp")
        .file("openbabel-patch/src/obmolecformat.cpp")
        .file("openbabel-patch/src/reactionfacade.cpp")
        .file("openbabel-patch/src/kekulize.cpp")
        .file("openbabel-patch/src/canon.cpp")
        .file("openbabel-patch/src/obfunctions.cpp")
        .file("openbabel-patch/src/griddata.cpp")
        .file("openbabel-patch/src/grid.cpp")
        .file("openbabel-patch/src/bondtyper.cpp")
        .file("openbabel-patch/src/stereo/cistrans.cpp")
        .file("openbabel-patch/src/stereo/tetrahedral.cpp")
        .file("openbabel-patch/src/stereo/tetranonplanar.cpp")
        .file("openbabel-patch/src/stereo/stereo.cpp")
        .file("openbabel-patch/src/stereo/perception.cpp")
        .file("openbabel-patch/src/stereo/facade.cpp")
        .file("openbabel-patch/src/stereo/squareplanar.cpp")
        .file("openbabel-patch/src/stereo/tetraplanar.cpp")
        .file("openbabel-patch/src/math/vector3.cpp")
        .file("openbabel-patch/src/math/matrix3x3.cpp")
        .file("openbabel-patch/src/math/spacegroup.cpp")
        .file("openbabel-patch/src/math/transform3d.cpp")
        .file("openbabel-patch/src/fingerprints/finger2.cpp")
        .file("openbabel-patch/src/fingerprints/finger3.cpp")
        .file("openbabel-patch/src/fingerprints/fingerecfp.cpp")
        .file("openbabel-patch/src/fingerprint.cpp")
        .file("openbabel-patch/src/forcefields/forcefielduff.cpp")
        .file("openbabel-patch/src/forcefields/forcefieldgaff.cpp")
        .file("openbabel-patch/src/forcefields/forcefieldmmff94.cpp")
        .file("openbabel-patch/src/forcefields/forcefieldghemical.cpp")
        .file("openbabel-patch/src/forcefield.cpp")
        .file("openbabel-patch/src/molchrg.cpp")
        // .file("openbabel-patch/src/forcefields/forcefieldmm2.cpp")  // compilation error when added
        .file("openbabel-patch/src/formats/smilesformat.cpp")
        .file("openbabel-patch/src/formats/xyzformat.cpp")
        .file("openbabel-patch/src/formats/gaussformat.cpp")
        .file("openbabel-patch/src/formats/gausscubeformat.cpp")
        .file("openbabel-patch/src/formats/gausszmatformat.cpp")
        .file("openbabel-patch/src/formats/fchkformat.cpp")
        .file("openbabel-patch/src/formats/turbomoleformat.cpp")
        .file("openbabel-patch/src/formats/daltonformat.cpp")
        .file("openbabel-patch/src/formats/orcaformat.cpp")
        .file("openbabel-patch/src/formats/siestaformat.cpp")
        .file("openbabel-patch/src/formats/mdlformat.cpp")
        .file("openbabel-patch/src/alias.cpp")
        .file("openbabel-patch/src/mcdlutil.cpp")
        .file("src/wrapper.cpp")
        .include(include)
        .include("src")
        .include("src/data")
        .include("openbabel-patch/include")
        .include("openbabel-patch/src/formats") // smilesvalence.h
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-overloaded-virtual")
        .flag_if_supported("-Wno-char-subscripts")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-extra")
        .flag_if_supported("-Wno-reorder")
        .flag_if_supported("-Wno-misleading-indentation")
        .flag_if_supported("-Wno-parentheses")
        // .flag("-Wno-c++11-extensions")
        .flag_if_supported("-Wno-unused-private-field")
        .compile("openbabel");

        println!("cargo:rerun-if-changed=src/lib.rs");
        println!("cargo:rerun-if-changed=src/wrapper.h");
        println!("cargo:rerun-if-changed=src/wrapper.cpp");
}
