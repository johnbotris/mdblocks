use std::env::args;
use std::io::{ stdin, Read };
use std::fs::File;

fn main() {
    let args = args().skip(1);

    let mut input = String::new();

    if args.len() == 0 {
        eprintln!("Reading from stdin. Type ^D to finish");
        if let Err(err) = stdin().lock().read_to_string(&mut input) {
            eprintln!("Error reading from stdin: {}", err);
        }
    }
    else {
        for path in args {
            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => { eprintln!("Couldn't open \"{}\": {}", path, err); continue }
            };
            if let Err(err) = file.read_to_string(&mut input) {
                eprintln!("Error reading \"{}\": {}", path, err);
            }
        }
    };

    let mut inside_code_block = false;
    let mut output = String::new();
    for line in input.lines() {
        if line.trim() == "```" { 
            inside_code_block = !inside_code_block;
            continue
        }
        if inside_code_block {
            output += line;
            output += "\n";
        }
    }

    println!("{}", output);
}

