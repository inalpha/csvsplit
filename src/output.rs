pub struct Output {
    w: csv::Writer<std::fs::File>,
    c: HashSet<String>,
}

impl Output {
    pub fn new(path: &str, header: &csv::ByteRecord) -> Self {
        let mut w = csv::Writer::from_writer(fs::File::create(path).unwrap());
        w.write_byte_record(header).unwrap();
        w.flush().unwrap();
        Output {
            w: w,
            c: HashSet::new(),
        }
    }

    pub fn write(&mut self, row: &csv::ByteRecord) {
        let unique = str::from_utf8(&row.get(0).unwrap()).unwrap().to_string();
        if !self.c.contains(&unique) {
            self.c.insert(unique);
            self.w.write_byte_record(row).unwrap();
        }
    }

    pub fn flush(&mut self) {
        self.w.flush().unwrap();
    }
}