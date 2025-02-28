use std::env;
use std::ffi::OsStr;
use std::fs::{remove_file, symlink_metadata};
use std::os::unix::fs;
use std::path::{Path, PathBuf};

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

    for entry in llvm_files.chain(clang_files).chain(
        specific_files
            .iter()
            .map(|s| Ok(PathBuf::from(s.to_string()))),
    ) {
        if let Ok(path) = entry {
            let path = Path::new(&path);
            if let Some(filename) = path.file_name().and_then(OsStr::to_str) {
                if filename.ends_with(&version) {
                    let new_filename = &filename[..filename.len() - version.len()];
                    let new_path = path.with_file_name(new_filename);

                    if let Ok(metadata) = symlink_metadata(&new_path) {
                        if metadata.file_type().is_symlink() {
                            if let Err(e) = remove_file(&new_path) {
                                eprintln!(
                                    "Failed to remove existing symlink {}: {}",
                                    new_path.display(),
                                    e
                                );
                                continue;
                            }
                        }
                    }

                    println!("Linking {} to {}", path.display(), new_path.display());
                    if let Err(e) = fs::symlink(&path, &new_path) {
                        eprintln!("Failed to create symlink: {}", e);
                    }
                }
            }
        }
    }
}
