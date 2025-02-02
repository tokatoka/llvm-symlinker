use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::env;
use std::os::unix::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <version>", args[0]);
        return;
    }
    let version = format!("-{}", args[1]);
    
    let llvm_files = glob::glob("/usr/bin/llvm*").unwrap();
    let clang_files = glob::glob("/usr/bin/clang*").unwrap();
    let specific_files = vec![
        format!("/usr/bin/llc{}", version),
        format!("/usr/bin/opt{}", version),
        format!("/usr/bin/lld{}", version),
    ];

    for entry in llvm_files.chain(clang_files).chain(specific_files.iter().map(|s| Ok(PathBuf::from(s.to_string())))) {
        if let Ok(path) = entry {
            let path = Path::new(&path);
            if let Some(filename) = path.file_name().and_then(OsStr::to_str) {
                if filename.ends_with(&version) {
                    let new_filename = &filename[..filename.len() - version.len()];
                    let new_path = path.with_file_name(new_filename);
                    
                    println!("Linking {} to {}", path.display(), new_path.display());
                    if let Err(e) = fs::symlink(&path, &new_path) {
                        eprintln!("Failed to create symlink: {}", e);
                    }
                }
            }
        }
    }
}