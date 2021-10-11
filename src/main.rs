use std::{
    env::{self, Args},
    ffi::OsStr,
    fs::{self},
    io::{self, stdout, BufWriter, StdoutLock, Write},
    os::unix::prelude::PermissionsExt,
};

struct Counts {
    dirs: i64,
    files: i64,
}

const DIR_C: &str = "\x1b[1;34m";
const EXE_C: &str = "\x1b[1;35m";
const SYM_C: &str = "\x1b[36m";
const NOM_C: &str = "\x1b[0m";
const ERR_C: &str = "\x1b[1;31m";

struct FileEntry {
    filename: String,
    is_dir: bool,
    is_sym: bool,
    is_exe: bool,
}

#[inline(never)]
fn walk(opts: &Opts, out: &mut BufWriter<StdoutLock>, dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
    let mut paths = match fs::read_dir(dir) {
        Ok(paths) => paths
            .filter_map(Result::ok)
            // .filter_map(|de| de.path().file_name().and_then(OsStr::to_str).map(|s| (s.to_string(), de.path().is_dir())))
            .filter_map(|de| {
                de.path().file_name().and_then(OsStr::to_str).map(|s| FileEntry {
                    filename: s.to_string(),
                    is_dir: de.path().is_dir(),
                    is_sym: de.file_type().map(|ft| ft.is_symlink()).unwrap_or(false),
                    is_exe: de.metadata().map(|m| (m.permissions().mode() & 0o111).ne(&0)).unwrap_or(false),
                })
            })
            .collect::<Vec<FileEntry>>(),
        Err(e) => {
            return writeln!(out, "{}\u{2514}\u{2500}\u{2500} {}{}{}", prefix, ERR_C, e, NOM_C);
        }
    };

    let mut index = paths.len();

    if opts.sort {
        paths.sort_by(|a, b| a.filename.cmp(&b.filename));
    };

    for fe in paths {
        index -= 1;

        if fe.filename.starts_with('.') {
            continue;
        }

        let color = if fe.is_dir {
            counts.dirs += 1;
            DIR_C
        } else {
            counts.files += 1;
            if fe.is_sym {
                SYM_C
            } else if fe.is_exe {
                EXE_C
            } else {
                NOM_C
            }
        };

        let (leader, trail) = if index.eq(&0) { ("\u{2514}", " ") } else { ("\u{251c}", "\u{2502}") };

        let _w = writeln!(out, "{}{}\u{2500}\u{2500} {}{}{}", prefix, leader, color, fe.filename, NOM_C);
        if fe.is_dir && !fe.is_sym {
            walk(opts, out, &format!("{}/{}", dir, fe.filename), &format!("{}{}   ", prefix, trail), counts)?;
        }
    }

    Ok(())
}

#[derive(Default)]
struct Opts {
    sort: bool,
}

fn parse_args(args: &mut Args) -> (Opts, String) {
    let mut opts = Opts { ..Default::default() };

    args.next();
    for arg in args {
        match arg.to_lowercase().as_ref() {
            "--sort" | "-sort" | "-s" => opts.sort = true,
            _ => return (opts, arg),
        }
    }

    (opts, '.'.to_string())
}

fn main() -> io::Result<()> {
    let (opts, dir) = parse_args(&mut env::args());

    let out = stdout();
    let out = out.lock();
    let mut out = BufWriter::new(out);

    let _w = writeln!(out, "{}{}{}", DIR_C, dir, NOM_C);

    let mut counts = Counts { dirs: 0, files: 0 };
    walk(&opts, &mut out, &dir, "", &mut counts)?;

    let _w = writeln!(out, "\n{} directories, {} files", counts.dirs, counts.files);

    Ok(())
}
