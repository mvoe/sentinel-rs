use std::io::{self, Write};
use colored::*;
use url::Url;

pub fn ask_for_target() -> String {
    loop {
        print!("{}", "🔍 Enter target URL (e.g. https://example.com): ".bright_cyan().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("{} {}", "✖ Failed to read input:".red(), e);
            continue;
        }

        let trimmed = input.trim();

        match Url::parse(trimmed) {
            Ok(_) => return trimmed.to_string(),
            Err(_) => {
                eprintln!("{} Invalid URL format.", "✖".red());
                continue;
            }
        }
    }
}
