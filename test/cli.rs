


#[test]
fn test_find_match() -> Result<(), Box<dyn std::error::Error>> {
    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run programsa
    use std::fs::File;
    use predicates::prelude::*; // Used for writing assertions


    let mut f = File::create("test_file.txt")?;
    f.write_all(b"This is a test file. Testing matching patterns.\npattern1\n pattern2 \n012934102934 \n\n\n temp");

    f.sync_data()?;
    let reader = std::io::BufReader::new(f);




}