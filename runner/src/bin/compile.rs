use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Error, Formatter};
use std::process::{Command, ExitStatus};

type Day = u8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(day) = std::env::args().nth(1) {
        let day = u8::from_str_radix(&day, 10)?;

        compile(day);
    }

    Ok(())
}

fn compile_all() -> Result<(), Error> {
    (1..=25)
        .filter(|&day| std::path::Path::new(&src(day)).exists())
        .collect::<Result<_, _>>()
}

fn compile(day: Day) -> Result<(), Error> {
    let mut child = Command::new("cargo")
        .args(&["build", "--release", "bin", &bin(day)])
        .spawn()?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        Err(Error::new((day, stderr.to_string())))
    }
}

fn cp_here(day: Day) -> Result<(), Box<dyn std::error::Error>> {
    let exe = exe(day);
    let _ = std::fs::copy(format!("target/release/{}", &exe), exe)?;

    Ok(())
}

fn bin(day: Day) -> String {
    format!("day{:0>2}", day)
}

fn exe(day: Day) -> String {
    #[cfg(target_os = "windows")]
    format!("{}.exe", &bin(day));

    #[cfg(not(target_os = "windows"))]
    format!("{}", &bin(day))
}

fn src(day: Day) -> String {
    format!("src/bin/{}.rs", &bin(day))
}

#[derive(Debug)]
struct Error {
    errors: BTreeMap<Day, String>,
}

impl Error {
    fn new(day_err: (Day, String)) -> Self {
        Self {
            errors: BTreeMap::from([day_err]),
        }
    }

    fn extend(mut self, err: Error) -> Self {
        self.errors.extend(err);
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (day, message) in self.errors {
            f.write_fmt(format_args!("Failed to compile day {}:\n{}", day, &message))?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {}
