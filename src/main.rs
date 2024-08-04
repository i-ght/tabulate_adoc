use core::panic;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines(path: &str) -> io::Result<Lines<BufReader<File>>>{
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn parse_adoc_notes(
    lines: &mut Lines<BufReader<File>>,
) -> io::Result<Vec<String>> {

    /* let mut notes: Vec<String> = Vec::with_capacity(8192);
    let mut note = Vec::with_capacity(8192);

    while let Some(line) = lines.next() {
        let line = line?;
        if line.starts_with("*") {
            note.push(
                line
                    .trim()
                    .to_string()
            );
            break;
        }
    }

    notes.push(note.join("\n"));
    note.clear();
    
    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if !note.is_empty() {
                notes.push(note.join("\n"));
            }
            
            note.clear();
            continue;
        }
        if !line.starts_with("*") {
            panic!("unhandled information!");
        }
        
        let information = line
            .trim();

        note.push(information.to_string());
    }
    
    if !note.is_empty() {
        notes.push(note.join("\n"));
        note.clear();
    }

    Ok(notes) */

    let mut notes = Vec::new();
    let mut note = Vec::new();
    let mut in_note = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            if in_note {
                notes.push(note.join("\n"));
                note.clear();
                in_note = false;
            }
            continue;
        }

        if line.starts_with("=") {
            continue;
        }

        if line.starts_with('*') {
            note.push(line.to_string());
            in_note = true;
            continue;
        } 
        
        panic!("unknown information found.");
    }

    // Add the last note if it exists
    if in_note {
        notes.push(note.join("\n"));
    }

    Ok(notes)
}

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() <= 1 {
        return Err(
            io::Error::new(io::ErrorKind::Other, "specify input file name in first arg.")
        );
    }

    let input_path = &argv[1];
    let path = Path::new(input_path);
    let stem = path.file_stem().unwrap();
    let stem = stem.to_str().unwrap();
    let output_file = format!("{}.csv", stem);

    let mut lines =read_lines(input_path)?;

    let notes = parse_adoc_notes(&mut lines)?;

    let mut writer = csv::Writer::from_path(&output_file)?;
    for note in notes {
        writer.write_record(&[note])?;
    }

    Ok(())
}
