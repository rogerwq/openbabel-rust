use std::io::Write;

fn replace_pattern(content: &str, pattern: &str, replacement: &str) -> String {
    let re = regex::Regex::new(pattern).unwrap();
    let after = re.replace_all(content, replacement);
    after.to_string()
}

fn remove_header_file(content: &str, include_header: &str) -> String {
    let re = regex::Regex::new(format!("(?P<h>{include_header})").as_str()).unwrap();
    let after = re.replace_all(content, "/** $h **/");
    after.to_string()
}

fn remove_function(content: &str, name: &str, lines: usize) -> String {
    let re = regex::Regex::new(format!("(?P<c>{name}\\(.*\\n(.*\\n){{{lines}}})").as_str()).unwrap();
    let after = re.replace_all(content, "/** $c **/");
    after.to_string()
}

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
        std::process::Command::new("git")
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

    // patch 
    let dir_ob_patch = std::path::Path::new("openbabel-patch");
    if dir_ob_patch.exists() {
        std::fs::remove_dir_all(dir_ob_patch).unwrap();
    }
    std::fs::create_dir(&dir_ob_patch).unwrap();
    let dir_ob_patch_include = dir_ob_patch.join("include").join("openbabel");
    let dir_ob_patch_src = dir_ob_patch.join("src");
    std::fs::create_dir_all(&dir_ob_patch_include).unwrap();
    std::fs::create_dir(&dir_ob_patch_src).unwrap();

    // list of files to be moved
    let source_files: Vec<(&str, &str, bool)> = vec![
        ("", "atom", true),
        ("", "alias", true),
        ("", "bond", true),
        ("", "bitvec", true),
        ("", "base", true), 
        ("", "builder", true), 
        ("", "bondtyper", false), 
        ("", "canon", true), 
        ("", "chains", true), 
        ("", "data", true), 
        ("", "data_utilities", false),  // no cpp file
        ("", "descriptor", true), // plugin related class
        ("", "distgeom", true), 
        ("", "elements", true),
        ("", "format", true), // plugin releated class 
        ("", "forcefield", true), // plugin releated class 
        ("", "generic", true), 
        ("", "graphsym", true), 
        ("", "internalcoord", false), 
        ("", "isomorphism", false), 
        ("", "kekulize", true), 
        ("", "lineend", false),  // no cpp file
        ("", "locale", true),
        ("", "mcdlutil", true),
        ("", "mol", true),
        ("", "molchrg", false),
        ("", "op", true), // plugin related class
        ("", "obconversion", true), // .h .cpp modified
        ("", "oberror", true),
        ("", "obutil", true),
        ("", "obiter", true),
        ("", "obfunctions", true),
        ("", "obmolecformat", true),
        ("", "parsmart", true),
        ("", "phmodel", true),
        ("", "rand", true), // no header file 
        ("", "reaction", false), // no cpp file
        ("", "reactionfacade", true),
        ("", "ring", true),
        ("", "rotamer", false),
        ("", "rotor", false),
        ("", "residue", true),
        ("", "shared_ptr", false), // no cpp file
        ("", "typer", true),
        ("", "tokenst", true), 
        ("", "text", false), // no cpp file
        ("", "transform", true), // no header file 
        ("math", "vector3", true),
        ("math", "spacegroup", false),
        ("math", "transform3d", false),
        ("math", "matrix3x3", true),
        ("stereo", "stereo", true),
        ("stereo", "cistrans", true),
        ("stereo", "tetraplanar", true),
        ("stereo", "tetranonplanar", true),
        ("stereo", "tetrahedral", true),
        ("stereo", "squareplanar", true),
        ("stereo", "perception", true), // no header file
        ("stereo", "facade", true), // no header file
        ("depict", "painter", false),
        ("depict", "svgpainter", true),
        // ("depict", "depict", true),
        ("formats", "smilesformat", true),
        // ("formats", "svgformat", true),
        // ("ops", "gen2D", true)
    ];

    // copy selected files
    let dir_ob = std::path::Path::new("openbabel");
    let dir_ob_include = dir_ob.join("include").join("openbabel");
    let dir_ob_src = dir_ob.join("src");
    for (dir_str, fn_str, copy_cpp) in source_files.iter() {
        let dir_include = dir_ob_patch_include.join(dir_str);
        let dir_src = dir_ob_patch_src.join(dir_str);
        let header_file_path = dir_ob_include.join(dir_str).join(format!("{}.h", fn_str));
        if header_file_path.exists() {
            std::fs::create_dir_all(&dir_include).unwrap();
            std::fs::copy(header_file_path, &dir_include.join(format!("{}.h", fn_str))).unwrap();
        }
        if *copy_cpp {
            std::fs::create_dir_all(&dir_src).unwrap();
            std::fs::copy(dir_ob_src.join(dir_str).join(format!("{}.cpp", fn_str)), &dir_src.join(format!("{}.cpp", fn_str))).unwrap();
        }
    }

    //  Patch by replacing
    for entry in walkdir::WalkDir::new(std::path::PathBuf::from("openbabel-patch")) {
        let entry = entry.unwrap();
        match entry.path().extension() {
            Some(ext) => {
                if ext == "cpp" || ext == "h" {
                    let file_content = std::fs::read_to_string(entry.path()).unwrap();
                    let mut after = file_content;
                    // remove plugins
                    after = remove_header_file(after.as_str(), "#include <openbabel/plugin.h>");
                    after = replace_pattern(after.as_str(), r"(?P<c>MAKE_PLUGIN\(.+\);*)", "/** $c **/");
                    after = replace_pattern(after.as_str(), r"(?P<c>: public OBPlugin)", "/** $c **/");
                    after = remove_function(after.as_str(), "static PluginMapType &FormatsMIMEMap", 4);
                    after = replace_pattern(after.as_str(), r"(?P<c>typedef OBPlugin::PluginIterator Formatpos;)", "/** $c **/");
                    after = replace_pattern(after.as_str(), r"(?P<c>static bool\s+GetNextFormat\(.*\);)", "/** $c **/");

                    if entry.file_name().to_str().unwrap() == "descriptor.h" {
                        after = replace_pattern(after.as_str(), r"(?P<c>const char\* TypeID\(\)\{.*\};)", "static OBDescriptor* FindType(const char* ID);\n$c");
                    }
                    if entry.file_name().to_str().unwrap() == "format.h" {
                        after = replace_pattern(after.as_str(), r"(?P<c>class OBCONV OBFormat.*\{.|\n*public:)", "$c\nstatic OBFormat* FindType(const char* ID);");
                    }
                    if entry.file_name().to_str().unwrap() == "forcefield.h" {
                        after = replace_pattern(after.as_str(), r"(?P<c>virtual OBForceField\* MakeNewInstance\(\)=0;)", "$c\nstatic OBForceField* FindType(const char* ID);");
                    }
                    if entry.file_name().to_str().unwrap() == "op.h" {
                        after = remove_function(after.as_str(), "static std::string OpOptions", 21);
                        after = replace_pattern(after.as_str(), r"(?P<c>typedef const std::map<std::string, std::string> OpMap)", "static OBOp* FindType(const char* ID);\n$c");
                    }

                    let mut file = std::fs::File::create(entry.path()).unwrap();
                    file.write(after.as_bytes()).unwrap();
                }
            }
            None => ()
        }
    }

    // Compiling
    cxx_build::bridge("src/lib.rs")
        .files(source_files.iter().filter(|(_, _, copy_cpp)| *copy_cpp).map(|(ds, fs, _)| dir_ob_patch.join("src").join(ds).join(format!("{fs}.cpp"))))
        .file("openbabel-patch/wrapper.cpp")  // srouce from "src/wrapper.cpp"
        .include(include)
        .include("src")
        .include("src/data")
        .include("openbabel-patch/include")
        .include("openbabel-patch/src/formats") // smilesvalence.h
        .include("openbabel/src/") // stereo/stereoutil.h
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
