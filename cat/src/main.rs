use std::env;
use std::fs;
use std::io::{self, BufRead};

/*
 * going to ignore -v, -u, -t, -e for now
 * 
 * I should have known this, but if you don't append a file
 * anythign you type will get echoed back to you because cat
 * is a madman
 */

#[derive(Debug,Clone)]
struct Arguments {
    count: bool,
    count_non_blank: bool,
    single_spaced: bool,
    files: Vec<String>
}


fn parse_aruments(args: Vec::<String>) -> Result<Arguments, String> {
    let mut pargs = Arguments {
       count: false,
       count_non_blank: false,
       single_spaced: false,
       files: vec![]
    };
    
    for (idx, arg) in args.iter().skip(1).enumerate() {
        if !arg.starts_with("-") {
            pargs.files = args[(idx + 1)..].to_vec();
            break;
        }

        match arg.as_str() {
            "-n" => pargs.count = true,
            "-b" => pargs.count_non_blank = true,
            "-s" => pargs.single_spaced = true,
            _    => { 
                return Err(format!("Illegal option: {}", arg));
           }
        };
    }

    Ok(pargs)
}


fn print_file_contents(file: &String, count: &bool, ignore_blank: &bool, single_spaced: &bool) {
    
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
    
    let mut current_count = 1;
    let mut prev_line_blank = false;
    for line in lines {
        if let Ok(l) = line {
            if *single_spaced {
                if prev_line_blank && l.is_empty() {
                    continue;
                } else if prev_line_blank && l.is_empty() == false {
                    prev_line_blank = true;
                } else {
                    prev_line_blank = false;
                }
            }

            if *ignore_blank && l.is_empty() == false {
               println!("{}\t {}", current_count, l);
               current_count += 1;
            } else if *count {
               println!("{}\t {}", current_count, l);
               current_count += 1;   
            } else {
                println!("{}", l);
            }
        }
    }
    
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let pargs = match parse_aruments(args) {
        Ok(args)   => args,
        Err(err) => {
            println!("{}", err);
            println!("Usage will go here.com");
            return
        }
    };

    for file in pargs.files {
        print_file_contents(&file, &pargs.count, &pargs.count_non_blank, &pargs.single_spaced);
    }
    
    return    
}

