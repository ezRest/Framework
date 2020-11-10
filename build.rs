extern crate yansi;
extern crate version_check;

use yansi::{Paint, Color::{Red, Yellow}};

const MIN_DATE: &'static str = "2020-1-1";
const MIN_VERSION: &'static str = "1.40.0-nightly";

macro_rules! err {
    ($version:expr, $date:expr, $msg:expr) => (
        eprintln!("{} {}", Red.paint("Error:").bold(), Paint::new($msg).bold());
        eprintln!("Installed version: {}", Yellow.paint(format!("{} ({})", $version, $date)));
        eprintln!("Minimum required:  {}", Yellow.paint(format!("{} ({})", MIN_VERSION, MIN_DATE)));
    )
}

fn main() {
    if let Some((version, channel, date)) = version_check::triple() {
        if !channel.supports_features() {
            err!(version, date, "ezRest (core) requires a 'nightly' version of rustc.");

            panic!("Aborting compilation due to incompatible compiler.")
        }

        if !version.at_least(MIN_VERSION) || !date.at_least(MIN_DATE) {
            err!(version, date, "ezRest (core) requires a more recent version of rustc.");
            panic!("Aborting compilation due to incompatible compiler.")
        }
    } else {
        println!("cargo:warning={}", "ezRest was unable to check rustc compiler compatibility.");
        println!("cargo:warning={}", "Build may fail due to incompatible rustc version.");
    }
}