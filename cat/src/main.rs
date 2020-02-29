use std::env;
use std::fs;
use std::io::{self, BufRead};

/* -n sounts lines
 * -b counts lines and skips blanks
 * -s makes the files single spaced
 *
 * Multiple files will be printed:
 * cat file1 file2 file3 
 *
 * Will say -n is now a file (its not, ya big goof), but print the rest
 * cat file1 -n file2
 *
 * going to ignore -v, -u, -t, -e for now
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file = match args.get(1) {
        Some(arg) => arg,
        _ => {
            println!("cat: <file> (no options, no problems)");
            return
        }
    };

    let file_metadata = match fs::metadata(file) {
        Ok(f)    => f,
        Err(err) => {
            println!("{}: {}", file, err);
            return
        }
    };

    if file_metadata.is_dir() {
        println!("{} is a directory", file);
        return 
    }

    let file_lines = match fs::File::open(file) {
        Ok(c) => c,
        Err(err) => {
          println!("Error reading {}: {}", file, err);
          return
        }
    };

    let lines = io::BufReader::new(file_lines).lines();

    for line in lines {
        if let Ok(l) = line {
            println!("{}", l);
        }
    }
}

