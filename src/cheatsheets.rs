use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use colored::{Color, ColoredString};

use crate::logger_cmd;
use crate::*;

pub fn print_cheat_sheets(input: &str, lhost: &str, rhost: &str) -> io::Result<()> {
    let cheatsheets_path = Path::new("/opt/winner/cheatsheets");

    // if search input matches with the file name, bring the role document
    let file_with_name = Path::new("/opt/winner/cheatsheets")
        .read_dir()?
        .filter(|dir| dir.as_ref().unwrap().path().is_dir())
        .flat_map(|dir| dir.as_ref().unwrap().path().read_dir().unwrap())
        .find(|file| {
            file.as_ref().unwrap().file_name().to_str().unwrap() == format!("{}.md", input)
        });

    match file_with_name {
        Some(file) => {
            let _ = find_on_file(input, &file.unwrap(), lhost, rhost, true);
        }
        None => {
            fs::read_dir(cheatsheets_path)?
                .filter(|dir| {
                    dir.as_ref().unwrap().path().file_name().unwrap() != ".git"
                        && dir.as_ref().unwrap().path().is_dir()
                })
                .flat_map(|dir| dir.as_ref().unwrap().path().read_dir().unwrap())
                .for_each(|file| {
                    let _ = find_on_file(input, &file.unwrap(), lhost, rhost, false);
                });
        }
    }

    Ok(())
}

// TODO NEED REFACTOR
fn find_on_file(
    input: &str,
    path: &DirEntry,
    lhost: &str,
    rhost: &str,
    show_all: bool,
) -> std::io::Result<()> {
    let file = File::open(path.path())?;
    let reader = BufReader::new(file);

    let binding = input.clone().to_lowercase();
    let input_list: Vec<&str> = binding.split(' ').collect();

    let mut subject: String = "Sumary".to_string();
    let mut first_time = true;
    let mut subject_changed = false;

    // Iterate all cheatsheet files
    for line in reader.lines() {
        let line_str = line?.to_string();

        if line_str.starts_with('#') {
            subject = line_str.clone();
            subject_changed = true;
        } else if line_str.starts_with('|')
            && !line_str.starts_with("| **Command")
            && !line_str.starts_with("------")
            && !line_str.starts_with("|------")
            && !line_str.starts_with("| **")
        {
            let row: Vec<&str> = line_str.split('|').collect();
            let mut command: String = row[1].replace('`', "").trim().to_string();
            let mut descr: String = row[2].trim().to_string();

            // if command.to_uppercase().contains(&input.to_uppercase()) || show_all {
            if input_list.clone().iter().all(|keyword| {
                command.clone().to_lowercase().contains(keyword)
                    || (command.clone().to_lowercase().contains(keyword) && input_list.len() > 1)
            }) || (show_all && input_list.len() == 1)
            {
                // replace RHOST and LHOST
                command = paint_and_replace(command.as_str(), "$RHOST", Color::Green, rhost);
                command = paint_and_replace(command.as_str(), "$LHOST", Color::Blue, lhost);

                // highline keywords
                for keyword in input_list.clone() {
                    command = highline(command.as_str(), keyword);
                    descr = highline(descr.as_str(), keyword);
                }

                if first_time {
                    print_first_time(path.path().file_name().unwrap().to_str().as_ref().unwrap());
                    first_time = false;
                }

                if subject_changed {
                    logger_summary!(subject.replace('#', "").to_string().trim());
                    subject_changed = false;
                }

                logger_cmd!(
                    format!("{}", "[~]"),
                    format!("{}", command),
                    format!(" ## {}", descr)
                );
            }

            // here is a row territory
        } else if line_str.starts_with("| **") && !line_str.starts_with("| **Command") {
            let row: Vec<&str> = line_str.split('|').collect();
            let command = row[1];

            subject_changed = true;
            subject = format!(
                "{} >> {}",
                subject.replace('#', "").to_string().trim(),
                command.replace(['*','|'], "").trim()
            );

            // logger_summary!();
        }
    }

    Ok(())
}

fn paint_and_replace(line: &str, word_to_paint: &str, color: Color, replace_str: &str) -> String {
    line.replace(
        word_to_paint,
        &ColoredString::from(replace_str)
            .color(color)
            .bold()
    )
    .to_string()
}

fn highline(line: &str, word_to_paint: &str) -> String {
    let highline = "\x1b[40m";
    let reset_color = "\x1b[0m";
    line.replace(
        word_to_paint,
        format!("{}{}{}", highline, word_to_paint, reset_color).as_str(),
    )
    .to_string()
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
