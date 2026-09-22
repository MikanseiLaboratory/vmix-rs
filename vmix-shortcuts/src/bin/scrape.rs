//! Download a vMix Shortcut Function Reference page and write `shortcuts.json`.
//!
//! ```text
//! cargo run -p vmix-shortcuts --features scrape --bin scrape -- --help-version 29 --output vmix-shortcuts/assets/shortcuts.json
//! ```

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut help_version: u32 = vmix_shortcuts::HELP_VERSION;
    let mut output = PathBuf::from("vmix-shortcuts/assets/shortcuts.json");
    let args: Vec<String> = env::args().skip(1).collect();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--help-version" => {
                index += 1;
                help_version = args
                    .get(index)
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(help_version);
            }
            "--output" => {
                index += 1;
                if let Some(path) = args.get(index) {
                    output = PathBuf::from(path);
                }
            }
            other => {
                eprintln!("unknown argument {other}");
                return ExitCode::from(2);
            }
        }
        index += 1;
    }

    let url = format!("https://www.vmix.com/help{help_version}/ShortcutFunctionReference.html");
    let html = match ureq::get(&url).call() {
        Ok(response) => match response.into_string() {
            Ok(body) => body,
            Err(error) => {
                eprintln!("failed to read {url}: {error}");
                return ExitCode::from(1);
            }
        },
        Err(error) => {
            eprintln!("failed to fetch {url}: {error}");
            return ExitCode::from(1);
        }
    };
    let shortcuts = vmix_shortcuts::parse_reference_html(&html);
    let json = match vmix_shortcuts::to_json(&shortcuts) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("failed to encode json: {error}");
            return ExitCode::from(1);
        }
    };
    if let Some(parent) = output.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            eprintln!("failed to create {}: {error}", parent.display());
            return ExitCode::from(1);
        }
    }
    if let Err(error) = fs::write(&output, format!("{json}\n")) {
        eprintln!("failed to write {}: {error}", output.display());
        return ExitCode::from(1);
    }
    println!("wrote {} functions to {}", shortcuts.len(), output.display());
    ExitCode::SUCCESS
}
