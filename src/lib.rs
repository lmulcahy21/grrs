use std::io::BufRead;


// WET - Write everything twice, more than that break apart into separate functionality
// modularize better
pub fn find_match(f: &mut impl std::io::Read, pattern: &str, writer: &mut impl std::io::Write) -> Result<(), Box<dyn std::error::Error>>{
    let buf_reader = std::io::BufReader::new(f);

    for read_result in buf_reader.lines(){
        let line = match read_result{
            Ok(line) => { line },
            Err(err) => { return Err(err.into())},
        };
        match match_pattern(writer, &pattern, &line){
            Err(err) => { return Err(err); }
            Ok(_) => {}, 
        }
    }
    Ok(())
}

fn match_pattern(writer: &mut impl std::io::Write, pattern: &str, line: &String) -> Result<(), Box<dyn std::error::Error>> {
    if line.contains(pattern){
        let write_res = writeln!(writer, "{}", &line);
        match write_res{
            Ok(_) => {},
            Err(err) => { return Err(err.into())}
        };
    }
    Ok(())
}





pub fn do_hard_work(){
    let mut val: i128 = 0i128;
    for i in 0i64..1000000{
        let a: i64 = i + 3;
        val += i128::from(a);
    }
    println!("{}", val);
}













