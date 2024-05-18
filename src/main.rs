use csv::Reader;
use std::{collections::HashMap, error::Error, io, process};

fn main() {
    if let Err(err) = csv2jsonl() {
        println!("{}", err);
        process::exit(1);
    }
}

fn csv2jsonl() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(io::stdin());
    let headers = reader.headers()?.to_owned();
    for result in reader.records() {
        let record = result?;
        let map: HashMap<&str, &str> = headers.iter().zip(record.iter()).collect();
        println!("{:?}", map);
    }
    Ok(())
}
