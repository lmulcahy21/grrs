


pub fn find_match(f: &std::fs::File, pattern: &str, writer: &mut impl std::io::Write) -> Result<(), Box<dyn std::error::Error>>{
    use std::io::BufRead;
    let buf_reader = std::io::BufReader::new(f);
    for read_result in buf_reader.lines(){
        let line = match read_result{
            Ok(line) => { line },
            Err(err) => { return Err(err.into())},
        };

        if line.contains(pattern){
            let write_res = writeln!(writer, "{}", line);
            match write_res{
                Ok(_) => {},
                Err(err) => { return Err(err.into())}
            };
        }
    }
    Ok(())
}













