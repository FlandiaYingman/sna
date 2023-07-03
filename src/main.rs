#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path, process::Command};
use std::os::windows::process::CommandExt;

use Root::{Multiple, Single, Zero};

const EXEC_PATH_CMD: &str = "7z.exe";
const EXEC_PATH_GUI: &str = "7zg.exe";

const OUTPUT_PATH: &str = "Path = ";

// Constant can be found in `winapi` or `windows` crates as well
//
// List of all process creation flags:
// https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
const CREATE_NO_WINDOW: u32 = 0x08000000; // Or `134217728u32`

enum Root {
    Zero,
    Single,
    Multiple,
}

fn main() {
    for archive_path_str in std::env::args().skip(1) {
        let archive_path = Path::new(archive_path_str.as_str());
        smart_extract(archive_path)
    }
}

fn smart_extract(archive_path: &Path) {
    let archive_path = fs::canonicalize(archive_path)
        .unwrap_or_else(|err| panic!("archive_path {} should be able to be canonicalized: {}", archive_path.display(), err));
    let root = probe(archive_path.as_path());

    match root {
        Zero => {}
        Single => {
            let destination = Path::new("./");
            extract(
                archive_path.as_path(),
                destination,
            )
        }
        Multiple => {
            let archive_stem = archive_path.file_stem()
                .unwrap_or_else(|| panic!("archive_path {} should have a file name", archive_path.display()));
            let destination = Path::new(archive_stem);
            extract(
                archive_path.as_path(),
                destination,
            )
        }
    }
}

fn probe(archive_path: &Path) -> Root {
    let archive_path_str = archive_path.to_str()
        .unwrap_or_else(|| panic!("archive_path {} should be able to convert to str", archive_path.display()));

    let output = Command::new(EXEC_PATH_CMD)
        // -slt : show technical information for l command
        // -sccUTF-8 : set charset for for console input/output
        .args(["l", "-slt", "-sccUTF-8", archive_path_str])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .unwrap_or_else(|err| panic!("process should exit successfully: {}", err));

    let output = String::from_utf8(output.stdout)
        .unwrap_or_else(|err| panic!("output.stdout should be able to convert to String: {}", err));

    let entries_num = output
        .lines()
        .filter(|x| x.starts_with(OUTPUT_PATH))
        .map(|x| x.trim_start_matches(OUTPUT_PATH))
        .filter(|x| x.matches(['/', '\\']).count() == 0)
        .count();

    if entries_num == 0 {
        Zero
    } else if entries_num == 1 {
        Single
    } else {
        Multiple
    }
}

fn extract(archive_path: &Path, destination_path: &Path) {
    let archive_path_str = archive_path.to_str()
        .unwrap_or_else(|| panic!("archive_path {} should be able to convert to str", archive_path.display()));
    let destination_path_str = destination_path.to_str()
        .unwrap_or_else(|| panic!("destination_path {} should be able to convert to str", destination_path.display()));

    Command::new(EXEC_PATH_GUI)
        // x : eXtract files with full paths
        // -o{Directory} : set Output directory
        .args(["x", format!("-o{}", destination_path_str).as_str(), archive_path_str])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .unwrap_or_else(|err| panic!("7z GUI {} should be executed successfully: {}", EXEC_PATH_GUI, err));
}
