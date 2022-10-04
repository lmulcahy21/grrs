
use clap::Parser;
use std::io::prelude::*;
use anyhow::{Context, Result};


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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
    let args: Cli = Cli::parse();
    println!("{} {}",args.pattern,args.path.display());

    /*
    let content: String = std::fs::read_to_string(&args.path).expect("Failed to read file"); -> initial implementation; not blazingly fast
    attempting some optimization through ReadBuf instead of reading whole file into memory
    */


    
    let f = std::fs::File::open(&args.path).with_context(|| format!("could not read file {}", args.path.display()))?;
        /*
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?; -> could create custom error for more helpful error message
        instead use anyhow library to add nicer error message
        */
    let mut reader = std::io::BufReader::new(f);

    let mut line: String = String::new();
    let mut len = reader.read_line(&mut line)?;
        


    /*
    Question mark on a Result enum turns into basically this:
    let len = match result {
        Ok(len) => { len },
        Err(error) => { panic!("Can't deal with {}, exit here", error); }
    };*/



    while len > 0 {
        if line.contains(&args.pattern) {
            println!("{}",line);
        }
        line.clear();
        len = reader.read_line(&mut line)?;
    }

    Ok(())

}
