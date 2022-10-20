#![allow(unused)]

use std::io::Write;




#[test]
fn test_find_match() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run programsa
    use std::fs::File;
    use predicates::prelude::*; // Used for writing assertions


    let mut f = File::create("test_file.txt")?;
    f.write_all(b"This is a test file. Testing matching patterns.\npattern1\n pattern2 \n012934102934 \n\n\n temp");

    f.sync_data()?;
    let mut writer = std::io::BufWriter::new(Vec::new());
    grrs::find_match(&f,"NOT_IN_STRING",&mut writer)?;
    assert_eq!(writer.into_inner()?, Vec::<u8>::new());
    //writer is now moved, so make new 
    let mut writer = std::io::BufWriter::new(Vec::new());
    grrs::find_match(&f,"temp",&mut writer)?;
    assert_eq!(&writer.into_inner()?, &Vec::from("temp"));


    Ok(())



}