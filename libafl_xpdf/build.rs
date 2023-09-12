use std::env;
use std::process::Command;

fn main() {
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
    let xpdf_dir = format!("{}/xpdf", cwd);

    // make clean
    Command::new("make")
        .arg("clean")
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Failed at make clean.");

    // "make install" clean
    Command::new("rm")
        .arg("-r")
        .arg("-v")
        .arg("-f")
        .arg(&format!("{}/install", xpdf_dir))
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Failed at 'make install' clean.");

    // Set environment variables
    env::set_var("LLVM_CONFIG", "llvm-config-11");
    env::set_var("CC", "$HOME/AFLplusplus/afl-clang-fast");
    env::set_var("CXX", "$HOME/AFLplusplus/afl-clang-fast++");

    // ./configure --prefix=/path/to/xpdf/install
    Command::new("./configure")
        .arg(&format!("--prefix={}/install", xpdf_dir))
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Failed at configure.");

    // make
    Command::new("make")
        .current_dir(xpdf_dir.clone())
        .status()
        .expect("Failed at make.");

    // make install
    Command::new("make")
        .arg("install")
        .current_dir(xpdf_dir)
        .status()
        .expect("Failed at 'make install'.");
}
