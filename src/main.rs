use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions; // opening file

use soup::prelude::*;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;



fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    // println!("{}", content);
    Ok(content)
}

fn main() {

    let content = read_file("messages.html").expect("File not found");

    let soup = Soup::new(content.as_str());

    let p = soup.tag("div").find().expect("Couldn't find tag 'p'");
    
    println!("{:?}", p.get("class"));

    let a = parse_stuff();
    print!("{:?}", a);

}


fn parse_stuff() -> Result<(), ParseError> {

    let ts = NaiveDateTime::parse_from_str("26.11.2019 21:55:36", "%d.%m.%Y %H:%M:%S")?;
    println!("{:?}", ts.timestamp());

    Ok(())
}