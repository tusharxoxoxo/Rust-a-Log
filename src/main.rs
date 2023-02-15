use colored::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Seek, SeekFrom};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut num_lines: usize = 10;
    if args.len() < 2 {
        println!("Usage: {} <log_file>", args[0]);
        //empty error return
        Err(io::Error::new(
            ErrorKind::NotFound,
            "Err: 01 | No file specified",
        ))?;
    }
    
    let path = Path::new(&args[1]).to_str().unwrap();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(io::Error::new(
                ErrorKind::Other,
                "Err: 69 | No such file - wrong file path",
            ))
        }
    };

    if args.len() == 3 {
        num_lines = match args.get(2) {
            Some(arg) => arg.parse::<usize>().unwrap_or(10),
            None => 10,
        };
    }
    dry_run(&file, num_lines)?;
    loop_run(&file)?;
    Ok(())
}

fn dry_run(path: &File, lines_sub: usize) -> io::Result<()> {
    let reader = BufReader::new(path);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let end: usize = lines.len();
    let start = if lines.len() > lines_sub {
        end - lines_sub
    } else {
        end - lines.len()
    };

    for line in lines[start..end].iter() {
        let str = line_parse(&line.to_string());
        println!("{}", str);
    }
    Ok(())
}

fn loop_run(file: &File) -> io::Result<()> {
    let mut reader = BufReader::new(file);

    loop {
        reader.seek(SeekFrom::End(0)).unwrap();
        let original_file_size = reader.seek(SeekFrom::Current(0)).unwrap();

        sleep(Duration::from_secs(1));

        let mut reader = BufReader::new(file);

        let current_file_size = reader.seek(SeekFrom::End(0)).unwrap();
        if current_file_size == original_file_size {
            continue;
        }

        reader.seek(SeekFrom::Start(original_file_size)).unwrap();

        for line in reader.lines() {
           let str = line_parse(&line.unwrap());
            println!("{}", str);
        }
    }
}

fn line_parse(line: &String) -> String{
    let log_level_start = match line.find("[") {
        Some(pos) => pos,
        None => {
            return line.white().to_string();
        }
    };
    let log_level_end = line.find("]").unwrap();
    let log_level = &line[log_level_start..log_level_end + 1];
    let date = &line[0..log_level_start].white();
    let message = &line[log_level_end + 1..].white().bold();

    let colored_level = match log_level {
        "[ERROR]" => log_level.red().bold(),
        "[WARNING]" => log_level.yellow().bold(),
        "[INFO]" => log_level.green().bold(),
        _ => log_level.normal(),
    };

    format!("{}{}{}", date, colored_level, message)
}
