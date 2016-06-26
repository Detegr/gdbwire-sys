use std::env;
use std::path::Path;
use std::process::Command;

struct CommandNotFoundError {
    cmd: String,
}

fn command_exists(cmd: &str) -> Result<(), CommandNotFoundError> {
    try!(Command::new(cmd)
        .output()
        .map_err(|_| CommandNotFoundError { cmd: format!("`{}` not found in PATH", cmd) }));
    Ok(())
}

fn check_autotools() -> Result<(), CommandNotFoundError> {
    try!(command_exists("aclocal"));
    try!(command_exists("autoconf"));
    try!(command_exists("autoheader"));
    try!(command_exists("automake"));
    try!(command_exists("libtoolize"));
    Ok(())
}

fn build_gdbwire() {
    if let Err(e) = check_autotools() {
        println!("{}", e.cmd);
        std::process::exit(1);
    }
    let autogen = Command::new("./autogen.sh")
        .current_dir("gdbwire")
        .status()
        .expect("Could not run autogen.sh");
    if autogen.code().unwrap_or(1) != 0 {
        println!("autogen.sh failed");
        std::process::exit(1);
    }
    let configure = Command::new("./configure")
        .arg(format!("--prefix={}",
                     env::var("OUT_DIR").expect("OUT_DIR not defined")))
        .current_dir("gdbwire")
        .status()
        .expect("Could not run configure");
    if configure.code().unwrap_or(1) != 0 {
        println!("configure failed");
        std::process::exit(1);
    }
    let make = Command::new("make")
        .current_dir("gdbwire")
        .arg(format!("-j{}", env::var("NUM_JOBS").expect("NUM_JOBS not defined")))
        .status()
        .expect("Could not run make");
    if make.code().unwrap_or(1) != 0 {
        println!("make failed");
        std::process::exit(1);
    }
    let make_install = Command::new("make")
        .current_dir("gdbwire")
        .arg("install")
        .status()
        .expect("Could not run make install");
    if make_install.code().unwrap_or(1) != 0 {
        println!("make install failed");
        std::process::exit(1);
    }
}

fn main() {
    if !Path::new("gdbwire/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not defined");
    let out_path = Path::new(&out_dir).join("lib");
    if !out_path.join("libgdbwire.a").exists() {
        build_gdbwire();
        let _ = std::fs::remove_file("ltmain.sh"); // TODO: Figure out why this gets copied here
    }
    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-lib=static=gdbwire");
}
