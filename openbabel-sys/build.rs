fn main() {
    if !std::path::Path::new("openbabel/.git").exists() {
        let _ = std::process::Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status().unwrap();
    }
}