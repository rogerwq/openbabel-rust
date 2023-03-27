use std::io::Write;

fn replace_pattern(content: &str, pattern: &str, replacement: &str) -> String {
    let re = regex::Regex::new(pattern).unwrap();
    let after = re.replace_all(content, replacement);
    after.to_string()
}

fn remove_header_file(content: &str, include_header: &str) -> String {
    let re = regex::Regex::new(format!("(?P<h>#include {include_header})").as_str()).unwrap();
    let after = re.replace_all(content, "/** $h **/");
    after.to_string()
}

fn add_header_file(content: &str, after_header: &str, another_header: &str) -> String {
    let re = regex::Regex::new(format!("(?P<h>#include {after_header})").as_str()).unwrap();
    let after = re.replace_all(content, format!("$h\n#include {another_header}").as_str());
    after.to_string()
}

fn remove_function(content: &str, name: &str, lines: usize) -> String {
    let re = regex::Regex::new(format!("(?P<c>{name}\\(.*\\n(.*\\n){{{lines}}})").as_str()).unwrap();
    let after = re.replace_all(content, "/** $c **/");
    after.to_string()
}

fn disable_function(content: &str, name: &str, lines: usize, new_body: &str) -> String {
    let re = regex::Regex::new(format!("(?P<c>(?P<n>{name}\\(.*\\n)(.*\\n){{{lines}}})").as_str()).unwrap();
    let after = re.replace_all(content, format!("/** $c **/ $n {{ {new_body} }}").as_str());
    after.to_string()
}

fn add_plugin_router(content: &str, category: &str, plugin_vec: &Vec<(&str, &str, &str, &str)>) -> String {
    let (func_declarations, find_type_branches): (Vec<String>, Vec<String>) = plugin_vec.iter()
        .filter(|(cat, _fp, _cls, _id)| *cat == category)
        .map(|(cat, _fp, cls, id)| {
            (
                format!("OB{cat}* new_{cls}();"),
                format!("else if (!strcmp(ID, \"{id}\")) {{ return new_{cls}(); }}")
            )
        })
        .unzip();
    let func_declaration = func_declarations.join("\n");
    let find_type_func = format!("OB{category}* OB{category}::FindType(const char* ID) {{
        if (!ID) {{ return nullptr; }}
        {}
        else {{ return nullptr; }}
    }}", find_type_branches.join("\n"));


    let re = regex::Regex::new(r"(?P<c>namespace OpenBabel[^{]*\{)").unwrap();
    let after = re.replace_all(content, format!("$c\n{func_declaration}\n{find_type_func}").as_str());
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
    let source_files: Vec<(&str, &str)> = vec![
        ("", "atom"),
        ("", "alias"),
        ("", "bond"),
        ("", "bitvec"),
        ("", "base"), 
        ("", "builder"), 
        ("", "bondtyper"), 
        ("", "canon"), 
        ("", "chains"), 
        ("", "data"), 
        ("", "data_utilities"),  // no cpp file
        ("", "descriptor"), // plugin related class
        ("", "distgeom"), 
        ("", "elements"),
        ("", "fingerprint"), // plugin releated class 
        ("", "format"), // plugin releated class 
        ("", "forcefield"), // plugin releated class 
        ("", "generic"), 
        ("", "graphsym"), 
        ("", "grid"), 
        ("", "griddata"), 
        ("", "internalcoord"), 
        ("", "isomorphism"), 
        ("", "kekulize"), 
        ("", "lineend"),  // no cpp file
        ("", "locale"),
        ("", "mcdlutil"),
        ("", "mol"),
        ("", "molchrg"),
        ("", "op"), // plugin related class
        ("", "obconversion"), // .h .cpp modified
        ("", "oberror"),
        ("", "obutil"),
        ("", "obiter"),
        ("", "obfunctions"),
        ("", "obmolecformat"),
        ("", "parsmart"),
        ("", "phmodel"),
        ("", "query"),
        ("", "rand"), // no header file 
        ("", "reaction"), // no cpp file
        ("", "reactionfacade"),
        ("", "ring"),
        ("", "rotamer"),
        ("", "rotor"),
        ("", "residue"),
        ("", "shared_ptr"), // no cpp file
        ("", "typer"),
        ("", "tokenst"), 
        ("", "text"), // no cpp file
        ("", "transform"), // no header file 
        ("math", "vector3"),
        ("math", "spacegroup"),
        ("math", "transform3d"),
        ("math", "matrix3x3"),
        ("stereo", "stereo"),
        ("stereo", "cistrans"),
        ("stereo", "tetraplanar"),
        ("stereo", "tetranonplanar"),
        ("stereo", "tetrahedral"),
        ("stereo", "squareplanar"),
        ("stereo", "perception"), // no header file
        ("stereo", "facade"), // no header file
        ("depict", "painter"),
        ("depict", "svgpainter"),
        // ("depict", "depict"),
        ("formats", "smilesformat"),
        // ("formats", "svgformat"),
        // ("ops", "gen2D")
    ];

    let plugins = vec![
        ("Format", "smilesformat.cpp", "CANSMIFormat", "can"),
        ("Format", "smilesformat.cpp", "FIXFormat", "fix"),
        ("Format", "smilesformat.cpp", "SMIFormat", "smi"),
    ];

    // copy selected files
    let dir_ob = std::path::Path::new("openbabel");
    let dir_ob_include = dir_ob.join("include").join("openbabel");
    let dir_ob_src = dir_ob.join("src");
    for (dir_str, fn_str) in source_files.iter() {
        let dir_include = dir_ob_patch_include.join(dir_str);
        let dir_src = dir_ob_patch_src.join(dir_str);
        let header_file_path = dir_ob_include.join(dir_str).join(format!("{}.h", fn_str));
        if header_file_path.exists() {
            std::fs::create_dir_all(&dir_include).unwrap();
            std::fs::copy(header_file_path, &dir_include.join(format!("{}.h", fn_str))).unwrap();
        }
        let cpp_file_path = dir_ob_src.join(dir_str).join(format!("{}.cpp", fn_str));
        if cpp_file_path.exists() {
            std::fs::create_dir_all(&dir_src).unwrap();
            std::fs::copy(cpp_file_path, &dir_src.join(format!("{}.cpp", fn_str))).unwrap();
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
                    after = remove_header_file(after.as_str(), "<openbabel/plugin.h>");
                    after = replace_pattern(after.as_str(), r"(?P<c>MAKE_PLUGIN\(.+\);*)", "/** $c **/");
                    after = replace_pattern(after.as_str(), r"(?P<c>: public OBPlugin)", "/** $c **/");
                    after = remove_function(after.as_str(), "static PluginMapType &FormatsMIMEMap", 4);
                    after = replace_pattern(after.as_str(), r"(?P<c>typedef OBPlugin::PluginIterator Formatpos;)", "/** $c **/");
                    after = replace_pattern(after.as_str(), r"(?P<c>static bool\s+GetNextFormat\(.*\);)", "/** $c **/");

                    if "descriptor.h" == entry.file_name().to_str().unwrap() {
                        after = replace_pattern(after.as_str(), r"(?P<c>const char\* TypeID\(\)\{.*\};)", "static OBDescriptor* FindType(const char* ID);\n$c");
                    }
                    if "descriptor.cpp" == entry.file_name().to_str().unwrap() {
                        after = disable_function(after.as_str(), "bool OBDescriptor::Display", 13, "return false;");
                        after = remove_function(after.as_str(), "double OBDescriptor::PredictAndSave", 19);
                        after = remove_function(after.as_str(), "void OBDescriptor::AddProperties", 12);
                    }
                    if "fingerprint.h" == entry.file_name().to_str().unwrap() {
                        after = replace_pattern(after.as_str(), r"(?P<c>virtual ~OBFingerprint\(\)\{\})", "static OBFingerprint* FindType(const char* ID);\n$c");
                    }
                    if "fingerprint.cpp" == entry.file_name().to_str().unwrap() {
                        after = replace_pattern(after.as_str(), r"(?P<c>_pFP->GetID\(\))\);", "/** $c **/ \"FP2\");");
                    }
                    if "format.h" == entry.file_name().to_str().unwrap() {
                        after = replace_pattern(after.as_str(), r"(?P<c>class OBCONV OBFormat.*\{.|\n*public:)", "$c\nstatic OBFormat* FindType(const char* ID);");
                    }
                    if "format.cpp" == entry.file_name().to_str().unwrap() {
                        after = add_header_file(after.as_str(), "<openbabel/babelconfig.h>", "<iostream>");
                        after = disable_function(after.as_str(), "int OBFormat::RegisterFormat", 12, "return 0;");
                        after = disable_function(after.as_str(), "OBFormat\\* OBFormat::FormatFromMIME", 6, "return nullptr;");
                        after = disable_function(after.as_str(), "bool OBFormat::Display", 54, "return false;");
                        after = add_plugin_router(after.as_str(), "Format", &plugins);
                    }
                    if "forcefield.h" == entry.file_name().to_str().unwrap() {
                        after = replace_pattern(after.as_str(), r"(?P<c>virtual OBForceField\* MakeNewInstance\(\)=0;)", "$c\nstatic OBForceField* FindType(const char* ID);");
                    }
                    if "op.h" == entry.file_name().to_str().unwrap() {
                        after = remove_function(after.as_str(), "static std::string OpOptions", 21);
                        after = replace_pattern(after.as_str(), r"(?P<c>typedef const std::map<std::string, std::string> OpMap)", "static OBOp* FindType(const char* ID);\n$c");
                    }
                    if "obconversion.cpp" == entry.file_name().to_str().unwrap() {
                        after = remove_function(after.as_str(), "std::vector<std::string> OBConversion::GetSupportedInputFormat", 5);
                        after = remove_function(after.as_str(), "std::vector<std::string> OBConversion::GetSupportedOutputFormat", 5);
                        after = remove_function(after.as_str(), "bool OBConversion::GetNextFormat", 28);
                    }
                    if "transform.cpp" == entry.file_name().to_str().unwrap() {
                        after = disable_function(after.as_str(), r"const char\* OBMol::ClassDescription", 32, "return \"\";");
                        after = disable_function(after.as_str(), r"OBBase\* OBMol::DoTransformations", 210, "return nullptr;");
                    }

                    for (cat, fp, cls, _) in plugins.iter() {
                        if *fp == entry.file_name().to_str().unwrap() {
                            after = replace_pattern(after.as_str(), format!("{cls} the{cls}").as_str(), format!("OB{cat}* new_{cls}() {{ return static_cast<OB{cat}*>(new {cls}()); }}").as_str());
                        }
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
        .files(source_files.iter()
            .filter(|(ds, fs)| dir_ob_patch.join("src").join(ds).join(format!("{fs}.cpp")).exists())
            .map(|(ds, fs)| dir_ob_patch.join("src").join(ds).join(format!("{fs}.cpp"))))
        .file("src/wrapper.cpp")  
        .include(include)
        .include("src")
        .include("src/data")
        .include("openbabel-patch/include")
        .include("openbabel/src/") // stereo/stereoutil.h
        .include("openbabel/src/formats") // smilesvalence.h
        .include("openbabel/src/stereo") // stereoutil.h
        .include("openbabel/src/math") // ../rand.h
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
