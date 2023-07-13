use std::{error::Error, io::Read};

use csv::ReaderBuilder;

use crate::SalaryRecord;

pub fn fetch_dataset(url: &str) -> Result<String, Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    Ok(content)
}

pub fn load_dataset(csv_data: &str) -> Result<Vec<SalaryRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let mut result = Vec::new();

    // Explicit iteration (instead of using `reader.deserialize::<SalaryRecord>()`)
    // so that it's easier to investigate the errors on the raw data.
    for record in reader.records() {
        let str_entry = record?;
        let sal_entry = str_entry.deserialize(None).unwrap();
        result.push(sal_entry);
    }

    Ok(result)
}
