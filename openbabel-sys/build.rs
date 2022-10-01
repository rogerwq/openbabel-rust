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

    if !std::path::Path::new("openbabel/.git").exists() {
        let _ = std::process::Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status().unwrap();
    }

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

    // Compiling
    cxx_build::bridge("src/lib.rs")
        .file("openbabel/src/base.cpp")
        .file("openbabel/src/atom.cpp")
        .file("openbabel/src/bond.cpp")
        .file("openbabel/src/oberror.cpp")
        .file("openbabel/src/tokenst.cpp")
        .file("openbabel/src/generic.cpp")
        .file("openbabel/src/rand.cpp")
        .file("openbabel/src/graphsym.cpp")
        .file("openbabel/src/ring.cpp")
        .file("openbabel/src/phmodel.cpp")
        .file("openbabel/src/obiter.cpp")
        .file("openbabel/src/builder.cpp")
        .file("openbabel/src/plugin.cpp")
        .file("openbabel/src/data.cpp")
        .file("openbabel/src/locale.cpp")
        .file("openbabel/src/obutil.cpp")
        .file("openbabel/src/descriptor.cpp")
        .file("openbabel/src/elements.cpp")
        .file("openbabel/src/typer.cpp")
        .file("openbabel/src/chains.cpp")
        .file("openbabel/src/bitvec.cpp")
        .file("openbabel/src/parsmart.cpp")
        .file("openbabel/src/residue.cpp")
        .file("openbabel/src/mol.cpp")
        .file("openbabel/src/transform.cpp")
        .file("openbabel/src/obconversion.cpp")
        .file("openbabel/src/format.cpp")
        .file("openbabel/src/obmolecformat.cpp")
        .file("openbabel/src/reactionfacade.cpp")
        .file("openbabel/src/kekulize.cpp")
        .file("openbabel/src/canon.cpp")
        .file("openbabel/src/obfunctions.cpp")
        .file("openbabel/src/bondtyper.cpp")
        .file("openbabel/src/griddata.cpp")
        .file("openbabel/src/grid.cpp")
        .file("openbabel/src/rotor.cpp")
        .file("openbabel/src/stereo/cistrans.cpp")
        .file("openbabel/src/stereo/tetrahedral.cpp")
        .file("openbabel/src/stereo/tetranonplanar.cpp")
        .file("openbabel/src/stereo/stereo.cpp")
        .file("openbabel/src/stereo/perception.cpp")
        .file("openbabel/src/stereo/facade.cpp")
        .file("openbabel/src/stereo/squareplanar.cpp")
        .file("openbabel/src/stereo/tetraplanar.cpp")
        .file("openbabel/src/math/vector3.cpp")
        .file("openbabel/src/math/matrix3x3.cpp")
        .file("openbabel/src/math/spacegroup.cpp")
        .file("openbabel/src/math/transform3d.cpp")
        .file("openbabel/src/formats/smilesformat.cpp")
        .file("openbabel/src/formats/xyzformat.cpp")
        .file("openbabel/src/formats/gaussformat.cpp")
        .file("openbabel/src/formats/gausscubeformat.cpp")
        .file("openbabel/src/formats/gausszmatformat.cpp")
        .file("openbabel/src/formats/fchkformat.cpp")
        .file("openbabel/src/formats/turbomoleformat.cpp")
        .file("openbabel/src/formats/daltonformat.cpp")
        .file("openbabel/src/formats/orcaformat.cpp")
        .file("openbabel/src/formats/siestaformat.cpp")
        .file("openbabel/src/fingerprint.cpp")
        .file("openbabel/src/fingerprints/finger2.cpp")
        .file("openbabel/src/fingerprints/finger3.cpp")
        .file("openbabel/src/fingerprints/fingerecfp.cpp")
        .file("src/wrapper.cpp")
        .include(include)
        .include("src")
        .include("src/data")
        .include("openbabel/include")
        .include("openbabel/src/formats") // smilesvalence.h
        .flag_if_supported("-std=c++14")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-variable")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-reorder-ctor")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-private-field")
        .flag("-Wno-c99-extensions")
        .flag("-Wno-extra-tokens")
        // .flag("-Wno-c++11-extensions")
        .compile("openbabel");

        println!("cargo:rerun-if-changed=src/lib.rs");
        println!("cargo:rerun-if-changed=src/wrapper.h");
        println!("cargo:rerun-if-changed=src/wrapper.cpp");
}