use std::{
    env, fs,
    io::{self, stdout, StdoutLock, Write},
};

struct Counts {
    dirs: i32,
    files: i32,
}

fn walk(out: &mut StdoutLock, dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
    let mut paths: Vec<_> = fs::read_dir(dir)?.map(|entry| entry.unwrap().path()).collect();
    let mut index = paths.len();

    paths.sort_by(|a, b| {
        let aname = a.file_name().unwrap().to_str().unwrap();
        let bname = b.file_name().unwrap().to_str().unwrap();
        aname.cmp(bname)
    });

    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        index -= 1;

        if name.starts_with(".") {
            continue;
        }

        if path.is_dir() {
            counts.dirs += 1;
        } else {
            counts.files += 1;
        }

        if index == 0 {
            let _ = writeln!(out, "{}└── {}", prefix, name);
            if path.is_dir() {
                walk(out, &format!("{}/{}", dir, name), &format!("{}    ", prefix), counts)?;
            }
        } else {
            let _ = writeln!(out, "{}├── {}", prefix, name);
            if path.is_dir() {
                walk(out, &format!("{}/{}", dir, name), &format!("{}│   ", prefix), counts)?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let out = stdout();
    let mut out = out.lock();

    let dir = env::args().nth(1).unwrap_or(".".to_string());
    let _ = writeln!(out, "{}", dir);

    let mut counts = Counts { dirs: 0, files: 0 };
    walk(&mut out, &dir, "", &mut counts)?;

    let _ = writeln!(out, "\n{} directories, {} files", counts.dirs, counts.files);

    let _ = out.flush();

    Ok(())
}