extern crate config;
extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use config::{Config, ConfigError, File};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use std::error::Error;
use std::path::Path;
use std::{env, fs, io, process, str};

type BoxedWriter = csv::Writer<Box<io::Write + 'static>>;

fn main() -> Result<(), Box<Error>> {
    let inputname: String = env::args().nth(1).unwrap_or_else(|| {
        println!("missing input file");
        process::exit(1);
    });

    let configname: String = env::args().nth(2).unwrap_or_else(|| {
        println!("missing setting file");
        process::exit(1);
    });

    let matches = get_matches(&configname).unwrap_or_else(|e| {
        println!("[settings] {}", e);
        process::exit(1);
    });

    let file = fs::File::open(Path::new(&inputname)).unwrap_or_else(|e| {
        println!("could not open input file {}", e);
        process::exit(1);
    });

    let mut rdr = csv::ReaderBuilder::new().from_reader(file);
    let headers = rdr.byte_headers()?.clone();
    let mut writers: HashMap<String, BoxedWriter> = HashMap::new();
    let mut row = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut row)? { 
        let val = str::from_utf8(row.get(14).unwrap()).unwrap();
        for (path, m) in matches.clone() {
            if (m.iter().any(|x| x == val)) {
                println!("-> {}", path);
                let mut wrt = match writers.entry(path.clone()) {
                    Entry::Occupied(w) => w.into_mut(),
                    Entry::Vacant(v) => {
                        let file: Box<io::Write+'static> = Box::new(fs::File::create(path.clone()).unwrap());
                        let mut w = csv::Writer::from_writer(file);
                        v.insert(w)
                    }
                };
                wrt.write_byte_record(&row);
            } else {
                println!("-> others.csv");
            }
        }
    }

    Ok(())
}

pub fn get_matches(path: &str) -> Result<HashMap<String, Vec<String>>, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name(path))?;
    s.get::<HashMap<String, Vec<String>>>("matches")
}

pub struct Output {
    w: csv::Writer<std::fs::File>,
}

impl Output {
    pub fn new(path: &str, header: &csv::ByteRecord) -> Self {
        let mut w = csv::Writer::from_writer(fs::File::create(path).unwrap());
        w.write_byte_record(header).unwrap();
        w.flush().unwrap();
        Output { w: w }
    }

    pub fn write(&mut self, row: &csv::ByteRecord) {
        self.w.write_byte_record(row).unwrap();
    }

    pub fn flush(&mut self) {
        self.w.flush().unwrap();
    }
}
