extern crate config;
extern crate csv;

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

    let key: usize = match env::args().nth(3) {
        Some(n) => {
            match n.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("could not parse column index");
                    process::exit(1);
                },
            }
        },

        None => {
            println!("missing column index");
            process::exit(1);
        }
    };

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
        let val = str::from_utf8(row.get(key).unwrap()).unwrap();
        let path = get_path(val, &matches);
        let wrt = match writers.entry(path.clone()) {
                Entry::Occupied(w) => w.into_mut(),
                Entry::Vacant(v) => {
                    let file = fs::File::create(path).unwrap_or_else(|e| {
                        println!("could not create output file {}", e);
                        process::exit(1);
                    });
                    let file: Box<io::Write+'static> = Box::new(file);
                    let mut w = csv::Writer::from_writer(file);
                    w.write_byte_record(&headers)?;
                    v.insert(w)
                }
            };
        wrt.write_byte_record(&row)?;
    }
    Ok(())
}

fn get_matches(path: &str) -> Result<HashMap<String, Vec<String>>, ConfigError> {
    let mut s = Config::new();
    s.merge(File::with_name(path))?;
    s.get::<HashMap<String, Vec<String>>>("matches")
}

fn get_path(val: &str, matches: &HashMap<String, Vec<String>>) -> String {
    for (path, m) in matches {
        if m.iter().any(|x| x == val) { return path.clone(); }
    };
    String::from("others.csv")
}