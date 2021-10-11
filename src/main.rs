#![feature(is_symlink)]

use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, stdout, BufWriter, StdoutLock, Write},
};

struct Counts {
    dirs: i32,
    files: i32,
}

#[inline(never)]
fn walk(out: &mut BufWriter<StdoutLock>, dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
    let mut paths = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|p| !p.path().is_symlink())
        .filter_map(|e| e.path().file_name().and_then(OsStr::to_str).map(|s| (s.to_string(), e.path().is_dir())))
        .collect::<Vec<(String, bool)>>();
    let mut index = paths.len();

    paths.sort_by(|a, b| a.0.cmp(&b.0));

    for (file_name, is_dir) in paths {
        index -= 1;

        if file_name.starts_with('.') {
            continue;
        }

        if is_dir {
            counts.dirs += 1;
        } else {
            counts.files += 1;
        }

        if index == 0 {
            let _w = write!(out, "{}\u{2514}\u{2500}\u{2500} {}\n", prefix, file_name);
            if is_dir {
                walk(out, &format!("{}/{}", dir, file_name), &format!("{}    ", prefix), counts)?;
            }
        } else {
            let _w = write!(out, "{}\u{251c}\u{2500}\u{2500} {}\n", prefix, file_name);
            if is_dir {
                walk(out, &format!("{}/{}", dir, file_name), &format!("{}\u{2502}   ", prefix), counts)?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let out = stdout();
    let out = out.lock();

    let mut out = BufWriter::new(out);

    let dir = env::args().nth(1).unwrap_or_else(|| '.'.to_string());
    let _w = write!(out, "{}\n", dir);

    let mut counts = Counts { dirs: 0, files: 0 };
    walk(&mut out, &dir, "", &mut counts)?;

    let _w = write!(out, "\n{} directories, {} files\n", counts.dirs, counts.files);

    Ok(())
}
