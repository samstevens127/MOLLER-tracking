use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut file = File::open(filename)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    Ok(());
}
