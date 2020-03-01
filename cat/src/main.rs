use std::env;
use std::fs;
use std::io::{self, BufRead};

/* -n counts lines
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
 * 
 * I should have known this, but if you don't append a file
 * anythign you type will get echoed back to you because cat
 * is a madman
 */

#[derive(Debug)]
struct Arguments {
    count: bool,
    count_non_blank: bool,
    single_spaced: bool,
    files: Vec<String>
}


fn parse_aruments(args: Vec::<String>) -> Option<Arguments> {
    let mut pargs = Arguments {
       count: false,
       count_non_blank: false,
       single_spaced: true,
       files: vec![]
    };
    
    for (idx, arg) in args.iter().skip(1).enumerate() {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-n" => pargs.count = true,
                "-b" => pargs.count_non_blank = true,
                "-s" => pargs.single_spaced = true,
                _    => { 
                    println!("Illegal option: {}", arg); 
                    return None
                }
            };
        } else {
            pargs.files = args[(idx + 1)..].to_vec();
            break;
        }
    }

    Some(pargs)
}


fn print_file_contents(file: &String) {
    
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
fn main() {
    let args: Vec<String> = env::args().collect();
    let pargs = match parse_aruments(args) {
        Some(args) => args,
        None => {
            println!("Listen up partner - soemthing bad happened.");
            return
        }
    };

    println!("Args: {:?}", pargs);
    for file in pargs.files {
        print_file_contents(&file);
    }
    
    return    
}

