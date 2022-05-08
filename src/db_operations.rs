use std::error::Error;
use csv::ReaderBuilder;


pub fn list(file_path: String) -> Result<(), Box<dyn Error>> {
    let mut reader_from_path = ReaderBuilder::new().from_path(file_path)?;
    for result in reader_from_path.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn filter_by_string(file_path: String, filter_string: &str) -> Result<(), Box<dyn Error>> {
    let mut reader_from_path = ReaderBuilder::new().from_path(file_path)?;
    for result in reader_from_path.records() {
        let record = result?;
        let mut found = false;
        // TODO: work with struct (deserialize) and 
        // use only the suitable field
        for field in &record {
            if field.contains(filter_string) {
                found = true;
                println!("{:?}", field);
            }
        }
        if !found {
            println!("No records with {} ", filter_string);
        }
    }
    Ok(())
}
