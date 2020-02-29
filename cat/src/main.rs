use std::env;
use std::fs;

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

    if(file_metadata.is_dir()) {
      println!("{} is a directory", file);
      return
    }

    let contents = match fs::read_to_string(file) {
      Ok(c)    => c,
      Err(err) => {
        println!("Seems like we got an error");
        return
      }
    };

    println!("{}", contents);
}
