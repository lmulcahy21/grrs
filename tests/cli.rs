
use assert_fs::prelude::*;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs


#[test]
fn test_find_match() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("TEMPTESTFILE.txt")?;
    file.write_str("hello world!\nthis is another line!\n")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("hello").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello world!"));


    Ok(())
}

#[test]
fn test_empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("TEMPTESTFILE.txt")?;
    file.write_str("hello world!\nthis is another line!")?;
    let mut vec_w: Vec<u8> = Vec::new();
    grrs::find_match(&mut std::fs::File::open(file.as_os_str())?,"",&mut vec_w)?;
    
    let s = String::from_utf8(vec_w)?;
    println!("{}",s);
    assert!(s == String::from("hello world!\nthis is another line!\n"));
    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("fake pattern").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));
    Ok(())
}