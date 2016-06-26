use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("gdbwire/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }
}
