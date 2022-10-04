
use clap::Parser;
use std::io::prelude::*;

#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    //The pattern to look for 
    pattern: String,
    //the path to look in
    path: std::path::PathBuf,
}
/*Note: There are a lot of custom attributes you can add to fields. 
For example, we added one to tell clap how to parse the PathBuf type. 
To say you want to use this field for the argument after -o or --output, you’d add #[arg(short = 'o', long = "output")]
. For more information, see the clap documentation. 
https://docs.rs/clap/
 */

fn main() {
    //https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
    let args: Cli = Cli::parse();
    println!("{} {}",args.pattern,args.path.display());

    //let content: String = std::fs::read_to_string(&args.path).expect("Failed to read file");
    //attempting some optimization through ReadBuf instead of reading whole file into memory
    let f = std::fs::File::open(&args.path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(f);

    let mut line: String = String::new();
    let mut len = reader.read_line(&mut line);

    while len.unwrap() > 0 {
        if line.contains(&args.pattern) {
            println!("{}",line);
        }
        line.clear();
        len = reader.read_line(&mut line);
    }

}
