#![allow(unused)]
use clap::Parser;
use std::{io::{prelude::*, Sink, BufWriter}, process::exit};
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

    let args = Cli::parse();

    let f = std::fs::File::open(&args.path)?;

    println!("{:?}", args);

    let mut writer = std::io::BufWriter::new(std::io::stdout());

    grrs::find_match(&f, &args.pattern, &mut writer);
    /* PROGRESS BAR
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        grrs::do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");*/


////Logging
/// 
/// 
/* 
    use log::{info,warn};
    env_logger::init();
    info!("Starting up");
    warn!("oops, nothing done");

    //env RUST_LOG=info cargo run
*/
    Ok(())



    /*
    Question mark on a Result enum turns into basically this:
    let len = match result {
        Ok(len) => { len },
        Err(error) => { panic!("Can't deal with {}, exit here", error); }
    };*/
        
    //let f = std::fs::File::open(&args.path).with_context(|| format!("could not read file {}", args.path.display()))?;
        /*
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?; -> could create custom error for more helpful error message
        instead use anyhow library to add nicer error message
        */
    //let reader = std::io::BufReader::new(f);


}