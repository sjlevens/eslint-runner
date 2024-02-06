use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use std::process::Command;
use rayon::prelude::*;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let current_dir = &args[1];

    let log_file_path = "./logs.txt";
    let output_file_path = "./files.txt";
    let regex_pattern = r"\/packages\/[\w\/-]+\/[\w-]+\.tsx";

    extract_and_write_paths(log_file_path, output_file_path, regex_pattern)?;
    run_eslint_fix(output_file_path, current_dir)?;

    Ok(())
}

fn extract_and_write_paths(log_file_path: &str, output_file_path: &str, regex_pattern: &str) -> io::Result<()> {

    let log_file = File::open(log_file_path)?;
    let reader = io::BufReader::new(log_file);

    let mut output_file = File::create(output_file_path)?;

    let re = Regex::new(regex_pattern).unwrap();

    for line in reader.lines() {
        let line = line?;
        for cap in re.captures_iter(&line) {
            if let Some(matched_path) = cap.get(0) {
                writeln!(output_file, ".{}", matched_path.as_str())?;
            }
        }
    }

    Ok(())
}

fn run_eslint_fix(file_list_path: &str, current_dir: &str) -> io::Result<()> {
    let file = File::open(file_list_path)?;
    let reader = io::BufReader::new(file);

    let file_paths: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    file_paths.par_iter().for_each(|file_path| {

        println!("👀 {}", file_path);

        let output = Command::new("yarn")
            .current_dir(current_dir)
            .args(&["run", "eslint", &file_path, "--fix"])
            .output()
            .expect("Failed to execute yarn");

        // Check if the command was executed successfully
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("❌ {}", stderr);
        } else {
            println!("✅ {}", file_path)
        }

    });


    Ok(())
}