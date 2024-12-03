use pathdiff::diff_paths;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::{env, io};

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                cb(&entry);
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day number as the first argument");
    }

    let day = &args[1];
    let day = format!("{:0>2}", day);

    let new_dir = format!("{}/src/day{}", env!("CARGO_MANIFEST_DIR"), day);

    println!("Creating directory: {}", new_dir);

    std::fs::create_dir_all(&new_dir).expect("Failed to create directory");

    let templ_dir = format!("{}/bin/generate/templ", env!("CARGO_MANIFEST_DIR"));
    visit_dirs(Path::new(&templ_dir.clone()), &|entry| {
        let path = entry.path();
        let relative_path = diff_paths(&path, templ_dir.clone()).unwrap();
        let new_entry = format!("{}/{}", new_dir, relative_path.to_str().unwrap());

        if path.is_dir() {
            std::fs::create_dir_all(&new_entry).expect("Failed to create directory");
            return;
        }

        let content = fs::read_to_string(&path).expect("Failed to read file");
        let content = content.replace("{day}", &day);
        fs::write(&new_entry, content).expect("Failed to write file");
    })
    .expect("Failed to visit directory");

    // add mod statement to lib.rs
    let lib_path = format!("{}/src/lib.rs", env!("CARGO_MANIFEST_DIR"));
    let content = fs::read_to_string(&lib_path).expect("Failed to read file");
    let new_mod = format!("pub mod day{};", day);
    let content = content.replace(
        "// add new modules here",
        &format!("{}\n// add new modules here", new_mod),
    );
    fs::write(&lib_path, content).expect("Failed to write file");
}
