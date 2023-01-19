use std::io::{Error, ErrorKind};
use std::io::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<Vec<PathBuf>> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();
    let mut paths: Vec<PathBuf> = Vec::new();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        paths.push(dest_path.clone());
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(paths)
}

fn count_chars(string: &str, character: &char) -> i32 {
    let mut count = 0;

    for c in string.chars() {
        if c.eq(character) {
            count += 1;
        }
    }

    count
}

fn main() -> Result<()> {
    let result = copy("../../proto", "src/proto")?;
    let mut paths: Vec<&Path> = result.iter().map(PathBuf::as_path).collect();
    paths.sort_by(|a, b| {
        let deep_count1 = count_chars(a.to_str().unwrap(), &'/');
        let deep_count2 = count_chars(b.to_str().unwrap(), &'/');

        deep_count2.cmp(&deep_count1)
    });

    for path in paths.iter() {
        println!("{:?}", path.to_str());
    }

    if paths.len() <= 0 {
        return Err(Error::new(ErrorKind::Unsupported, "No Proto file found"));
    }

    prost_build::compile_protos(&paths[..], &["src/"])?;
    Ok(())
}
