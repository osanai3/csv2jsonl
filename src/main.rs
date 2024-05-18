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
        let mut map = HashMap::<&str, &str>::new();
        let record = result?;
        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header, field);
        }
        println!("{:?}", map);
    }
    Ok(())
}
