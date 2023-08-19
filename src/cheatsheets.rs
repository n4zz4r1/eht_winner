use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use colored::{Color, ColoredString};

use crate::*;
use crate::logger_cmd;

pub fn print_cheat_sheets(input: &str, lhost: &str, rhost: &str) -> std::io::Result<()> {
    let cheatsheets_path = Path::new("/opt/winner/cheatsheets");

    // 1. First level entry
    // cheatsheets_path?


    for first_level_entry in fs::read_dir(cheatsheets_path)? {
        let entry = first_level_entry?;

        if entry.file_name() != ".git" && entry.path().is_dir() {
            for second_level_entry in entry.path().read_dir()? {
                let md_entry = second_level_entry?;
                let _ = find_on_file(input, &md_entry, lhost, rhost);
            }
        }
    }

    Ok(())
}

fn find_on_file(input: &str, path: &DirEntry, lhost: &str, rhost: &str) -> std::io::Result<()> {

    let file = File::open(path.path())?;
    let reader = BufReader::new(file);

    let mut subject: String = "|- Sumary".to_string();
    let mut first_time = true;
    let mut subject_changed = false;

    for line in reader.lines() {
        let line_str = line?.to_string();

        if line_str.starts_with("#") {
            subject = line_str.clone();
            subject_changed = true;
        } else if line_str.starts_with("|") && !line_str.starts_with("| **Command") && !line_str.starts_with("------") && !line_str.starts_with("|------") && !line_str.starts_with("| **") {
            let row: Vec<&str> = line_str.split('|').collect();
            let mut command: String = row[1].replace("`","").trim().to_string();
            let descr = row[2].trim().to_string();

            if command.to_uppercase().contains(&input.to_uppercase()) {
                // command = paint(command, input, Color::Blue);
                // command = paint( command.as_str(), input, Color::BrightYellow);
                command = highline(command.as_str(), input);

                if first_time {
                    print_first_time(path.path().file_name().unwrap().to_str().as_ref().unwrap());
                    first_time = false;
                }

                if subject_changed {
                    logger_summary!(subject.replace("#","").to_string().trim());
                    subject_changed = false;
                }
                // println!("{}", format!(" ## {}", descr).purple());

                logger_cmd!(format!("{}", "[~]"),format!("{}", command),format!(" ## {}", descr));
            }

            // here is a row territory
        } else if line_str.starts_with("| **") && !line_str.starts_with("| **Command") {
            let row: Vec<&str> = line_str.split('|').collect();
            let command = row[1];

            subject_changed = true;
            subject = format!("{} >> {}", subject.replace("#","").to_string().trim(), command.replace("*", "").replace("|", "").trim());

            // logger_summary!();
        }
    }

    Ok(())
}

fn paint(line:&str, word_to_paint: &str, color: Color) -> String {

    line.replace(word_to_paint, &ColoredString::from(word_to_paint).color(color).bold().to_string()).to_string()
}
fn highline(line:&str, word_to_paint: &str) -> String {
    let highline = "\x1b[40m";
    let reset_color = "\x1b[0m";

    line.replace(word_to_paint, format!("{}{}{}", highline, word_to_paint, reset_color).as_str()).to_string()

}

fn print_first_time(path: &str) {
    println!(" ┌──────────────────────────────┐   ");
    println!(
        " │  {}{:<24}  │",
        Icons::Medal.to_string().bold().yellow(),
        path
    );
    println!(" └──────────────────────────────┘   ");
}