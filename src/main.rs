use std::{path::Path, process::Command};

fn main() {
    println!("{:?}", probe_roots("zip64.zip"))
}

#[derive(Debug)]
enum Root {
    Single,
    Multiple,
    None,
}

static EXEC_PATH: &str = "C:/Program Files/7-Zip-Zstandard/7z.exe";
fn probe_roots(archive_path: &str) -> Root {
    // Call Archiver to get the roots of the archive
    let output = String::from_utf8(
        Command::new(EXEC_PATH)
            .args(["l", "-slt", "-ba", archive_path])
            .output()
            .expect("failed to execute process")
            .stdout,
    )
    .expect("failed to convert output to string");

    // Parse the output to get the roots
    let entries = output.lines().filter_map(|x| {
        use path_slash::PathExt as _;
        if x.starts_with("Path = ") {
            return Option::Some(
                Path::new(x.trim_start_matches("Path = "))
                    .to_slash()
                    .expect("failed to convert path to slash"),
            );
        }
        return Option::None;
    });

    // Check if the archive is a single root or not
    let mut root_occurred = false;
    for path in entries {
        if path.matches("/").count() == 0 {
            if !root_occurred {
                // this is the first root file or directory occurrence
                root_occurred = true
            } else {
                // this is not the first root file or directory occurrence,
                // so we know there is more than one root
                return Root::Multiple;
            }
        }
    }
    if root_occurred {
        return Root::Single;
    } else {
        return Root::None;
    }
}
