pub struct Input {
    indices: Vec<Option<usize>>,
    rdr: csv::Reader<std::fs::File>,
    row: csv::ByteRecord,
}

impl Input {
    pub fn new(path: &str, columns: &Vec<Vec<String>>) -> Self {
        let path = Path::new(path);
        let file = fs::File::open(path).unwrap();
        let mut rdr = csv::ReaderBuilder::new().from_reader(file);
        let mut indices: Vec<Option<usize>> = vec![None; columns.len()];

        {
            let headers = rdr.byte_headers().unwrap();
            for (i, header) in headers.iter().enumerate() {
                for (j, matches) in columns.iter().enumerate() {
                    if matches.contains(&str::from_utf8(header).unwrap().to_string()) {
                        indices[j] = Some(i);
                    }
                }
            }
        }

        Input {
            indices: indices,
            rdr: rdr,
            row: csv::ByteRecord::new(),
        }
    }

    pub fn next(&mut self) -> Option<csv::ByteRecord> {
        match self.rdr.read_byte_record(&mut self.row) {
            Ok(true) => {
                let mut row = csv::ByteRecord::new();
                for i in &self.indices {
                    row.push_field(match i {
                        None => b"",
                        Some(i) => &self.row.get(*i).unwrap(),
                    })
                }
                Some(row)
            }
            Ok(false) => None,
            Err(_) => None,
        }
    }
}
