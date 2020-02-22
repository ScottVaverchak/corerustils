use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = match args.get(1) {
        Some(x) => x,
        None => "y"
    };
    
    loop {
        println!("{}", output);
    }
}
