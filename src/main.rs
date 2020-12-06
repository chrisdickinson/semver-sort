use structopt::StructOpt;
use std::io::prelude::*;
use std::collections::BTreeSet;
use semver::Version;

#[derive(StructOpt)]
#[structopt(name = "semver-sort", about = "Sort input according to semver precedence rules")]
struct Flags {
    #[structopt(short="k", long, default_value="1")]
    key: usize,
    #[structopt(short="r", long, help = "Reverse the sort order: output higher versions first")]
    reverse: bool,
    #[structopt(short="u", long, help = "Only output unique lines")]
    unique: bool,
    #[structopt(short="p", long, help = "Include prerelease versions in output")]
    prerelease: bool
}

static mut UNIQUE: bool = false;

struct Line {
    idx: usize,
    source: String,
    semver: Version
}

impl std::cmp::PartialEq for Line {
    fn eq(&self, rhs: &Self) -> bool {
        if unsafe { UNIQUE } {
            self.source == rhs.source
        } else {
            self.idx == rhs.idx
        }
    }
}

impl std::cmp::PartialOrd for Line {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.semver.partial_cmp(&rhs.semver)
    }
}

impl std::cmp::Eq for Line {
}

impl std::cmp::Ord for Line {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.semver.cmp(&rhs.semver)
    }
}

fn write_out(iter: impl std::iter::Iterator<Item=Line>) -> Result<(), Box<dyn std::error::Error>> {
    for line in iter {
        std::io::stdout().write_all(line.source.as_bytes())?;
        std::io::stdout().write_all(b"\n")?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::from_args();

    unsafe { UNIQUE = flags.unique };
    let mut output = BTreeSet::new();
    let stdin = std::io::stdin();
    let invalid: Version = "0.0.0-0".parse().unwrap();
    for (idx, line) in stdin.lock().lines().enumerate() {
        let line = line?;

        let mut bits = line.split_whitespace();
        let version: Version = bits.nth(flags.key - 1).and_then(|xs| {
            if xs.starts_with('v') {
                xs[1..].parse().ok()
            } else {
                xs.parse().ok()
            }
        }).unwrap_or_else(|| {
            invalid.clone()
        });

        if version.is_prerelease() && !flags.prerelease {
            continue
        }

        output.insert(Line {
            idx,
            source: line,
            semver: version
        });
    }

    if flags.reverse {
        write_out(output.into_iter().rev())?;
    } else {
        write_out(output.into_iter())?;
    }

    Ok(())
}
