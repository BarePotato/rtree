use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, stdout, StdoutLock, Write},
    path::PathBuf,
};

struct Counts {
    dirs: i32,
    files: i32,
}

fn walk(out: &mut StdoutLock, dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
    let mut paths = fs::read_dir(dir)?.filter_map(Result::ok).map(|e| e.path()).collect::<Vec<PathBuf>>();
    let mut index = paths.len();

    paths.sort_by(|a, b| {
        let a_name = a.file_name();
        let b_name = b.file_name();
        a_name.cmp(&b_name)
    });

    for path in paths {
        let name = if let Some(name) = path.file_name().and_then(OsStr::to_str) {
            name
        } else {
            continue;
        };

        index -= 1;

        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            counts.dirs += 1;
        } else {
            counts.files += 1;
        }

        if index == 0 {
            let _w = writeln!(out, "{}\u{2514}\u{2500}\u{2500} {}", prefix, name);
            if path.is_dir() {
                walk(out, &format!("{}/{}", dir, name), &format!("{}    ", prefix), counts)?;
            }
        } else {
            let _w = writeln!(out, "{}\u{251c}\u{2500}\u{2500} {}", prefix, name);
            if path.is_dir() {
                walk(out, &format!("{}/{}", dir, name), &format!("{}\u{2502}   ", prefix), counts)?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let out = stdout();
    let mut out = out.lock();

    let dir = env::args().nth(1).unwrap_or_else(|| '.'.to_string());
    let _w = writeln!(out, "{}", dir);

    let mut counts = Counts { dirs: 0, files: 0 };
    walk(&mut out, &dir, "", &mut counts)?;

    let _w = writeln!(out, "\n{} directories, {} files", counts.dirs, counts.files);

    let _w = out.flush();

    Ok(())
}
