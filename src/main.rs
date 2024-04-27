use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A Useful utility for splitting files, and optionally removing the first header line.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1000)]
    lines: u32,

    /// Skip the first line?
    #[arg(short, long)]
    skip_first: bool,
}

fn new_filename(file: &String, num : u32) -> String {
    // Split the filename into name and extension parts
    let parts: Vec<&str> = file.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        // If there's an extension, format the new filename with the number before the extension
        format!("{}_{}.{}", parts[1], num, parts[0])
    } else {
        // If there's no extension, just append the number to the end
        format!("{}_{}", file, num)
    }
}

fn split_file(filename: &String, lines: u32, skip_first: bool) -> io::Result<()>  {
    let mut cur_line : u32 = 0;
    let mut line_temp : u32 = 0;
    let mut cur_file_idx : u32 = 0;

    // Create a new file for writing
    let mut file = File::create(new_filename(filename, cur_file_idx))?;

    // Create a buffered writer to write to the file
    let mut writer = BufWriter::new(file);

    println!("Opening file: {}", filename);

    for line in my_reader::BufReader::open(filename)? {
        if cur_line == 0 && skip_first {
            // skip the first line
        } else {
            line_temp += 1;
            writer.write_all(line?.as_bytes())?;
            if line_temp >= lines {
                // create a new file
                line_temp = 0;

                // increment the file index
                cur_file_idx += 1;

                // Flush the writer to ensure all data is written to disk
                writer.flush()?;

                file = File::create(new_filename(filename, cur_file_idx))?;
                writer = BufWriter::new(file);
            }
        }
        cur_line += 1;
    }

    Ok(())
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}

fn main() {
    println!("Splitfile utility v{VERSION}");
    println!("");

    let args = Args::parse();

    println!("Splitting {} into files of {} lines each", args.file, args.lines);
    println!("{} lines will be skipped", if args.skip_first { 1 } else { 0 });

    match split_file(&args.file, args.lines, args.skip_first) {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::env;
    use std::fs;
    use std::fs::remove_file;
    use glob::glob;

    fn check_file_size(filename : &String, expected_size : u64) {
        match fs::metadata(filename) {
            Ok(metadata) => {
                let file_size = metadata.len();  // Retrieves the file size in bytes
                assert!(file_size == expected_size, "Invalid file size: {} : {} expected: {}", filename, file_size, expected_size);
            },
            Err(e) => panic!("Failed to get file metadata: {:?}", e),
        }
    }

    fn delete_files_by_pattern(pattern: &str) -> Result<(), std::io::Error> {
        for entry in glob(pattern).unwrap() {
            if let Ok(path) = entry {
                if let Err(err) = remove_file(path) {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn test_basic_split() {
        // Setup
        let lines_per_file = 10;
        let skip_first = true;
        let default_str = "Invalid path";

        let cwd = env::current_dir();
        let current_dir = cwd.unwrap();
        println!("Current working directory: {}", current_dir.display());

        // Create test file names
        let filename_path = current_dir.join("tests/test.csv");
        let file0_path = current_dir.join("tests/test_0.csv");
        let file1_path = current_dir.join("tests/test_1.csv");
        let file2_path = current_dir.join("tests/test_2.csv");
        let pattern = current_dir.join("tests/test_*.csv").to_str().unwrap().to_string();

        if let Err(_) = delete_files_by_pattern(pattern.as_str()) {
            panic!("Failed to delete test files");
        }

        let filename = filename_path.to_str().unwrap_or(default_str).to_string();
        let file0 = file0_path.to_str().unwrap_or(default_str).to_string();
        let file1 = file1_path.to_str().unwrap_or(default_str).to_string();
        let file2 = file2_path.to_str().unwrap_or(default_str).to_string();

        // Execute
        let result = split_file(&filename, lines_per_file, skip_first);

        // Verify
        assert!(result.is_ok() , "Error splitting file: {}", result.err().unwrap());
        assert!(Path::new(&file0).exists(), "File does not exist: {}", file0);
        assert!(Path::new(&file1).exists(), "File does not exist: {}", file1);
        assert!(Path::new(&file2).exists(), "File does not exist: {}", file2);

        check_file_size(&file0, 203);
        check_file_size(&file1, 230);
        check_file_size(&file2, 115);

    }

    // Other tests go here
}

