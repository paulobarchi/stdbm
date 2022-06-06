use std::error::Error;
use csv::ReaderBuilder;
use std::fs::OpenOptions;
use crate::data_structures::{
    Sentence, Tag, SentenceTag, Print};

pub fn list(file_path: String) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn filter_by_string(
    sentence_file_path: String, 
    filter_string: &str
) -> Result<(), Box<dyn Error>> {
    
    let mut reader = ReaderBuilder::new().from_path(sentence_file_path)?;
    let mut found: bool = false;
    for result in reader.deserialize() {
         let record: Sentence = result?;
        if record.sentence.contains(filter_string) {
            found = true;
            record.print();
        }
    }
    if !found {
        panic!("No sentence with {}.", filter_string);
    }
    Ok(())
}

pub fn filter_by_tag(
    sentence_file_path: String, 
    tag_file_path: String, 
    sentence_tag_file_path: String, 
    filter_tag: &str
) -> Result<(), Box<dyn Error>> {

    let mut found_tag: bool = false;
    let mut found_sentence: bool = false;
    let mut id_tag: String = String::new();
    let mut id_sentences: Vec<String> = Vec::new();
    
    // search for tag
    let mut tag_reader = ReaderBuilder::new().from_path(
        tag_file_path)?;
    for result in tag_reader.deserialize() {
        let record: Tag = result?;
        if record.tag.contains(filter_tag) {
            found_tag = true;
            id_tag = record.id;
        }
    }
    if !found_tag {
        panic!("There are no tags with {}.", filter_tag);
    }
    
    // retrieve sentence_ids that have such tag
    let mut sentence_tag_reader = ReaderBuilder::new().from_path(
        sentence_tag_file_path)?;
    for result in sentence_tag_reader.deserialize() {
            let record: SentenceTag = result?;
        if record.id_tag == id_tag {
            found_sentence = true;
            id_sentences.push(record.id_sentence);
        }
    }
    if found_sentence {
    let mut sentence_reader = ReaderBuilder::new().from_path(
    sentence_file_path)?;
        for result in sentence_reader.deserialize() {
            let record: Sentence = result?;
            for sentence_id in &id_sentences {
                if &record.id == sentence_id {
                    record.print();
                }
            }
        }
    }
    if !found_sentence {
        panic!("No sentence with tag content: {}.", filter_tag);
    }
    Ok(())
}

pub fn append_record_to_file(
    file_path: String, new_register: &[String]
) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path.clone())
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(new_register)?;
    wtr.flush()?;
    Ok(())
}


pub fn add_sentence(
    file_path: String, new_register: &str
) -> Result<(), Box<dyn Error>> {
    
    let mut reader = ReaderBuilder::new().from_path(file_path.clone())?;    
    let mut largest_id: i32 = -1;

    // check if already exists
    for result in reader.deserialize() {
        let record: Sentence = result?;
        if record.sentence == new_register {
            panic!("{} is already a Sentence", new_register);
        }
        let id_int = record.id.parse::<i32>().unwrap();
        if id_int > largest_id {
            largest_id = id_int;
        }
    }
    // append register
    let curr_id: i32 = largest_id + 1;
    if let Err(err) = append_record_to_file(file_path, &[curr_id.to_string(), new_register.to_string()]) {
        panic!("Error appending to file: {}", err);
    }
    Ok(())
}

pub fn add_tag(
    file_path: String, new_register: &str
) -> Result<(), Box<dyn Error>> {

    let mut reader = ReaderBuilder::new().from_path(file_path.clone())?;
    let mut largest_id: i32 = -1;
    
    // check if already exists
    for result in reader.deserialize() {
        let record: Tag = result?;
        if record.tag == new_register {
            panic!("{} is already a Tag", new_register);
        }
        let id_int = record.id.parse::<i32>().unwrap();
        if id_int > largest_id {
            largest_id = id_int;
        }
    }
    // append register
    let curr_id: i32 = largest_id + 1;
    if let Err(err) = append_record_to_file(
        file_path, &[curr_id.to_string(), new_register.to_string()]) 
    {
        panic!("Error appending to file: {}", err);
    }
    
    Ok(())
}

pub fn add(
    file_path: String, 
    new_register: &str
) -> Result<(), Box<dyn Error>> {
    if file_path.contains("tags") {
        if let Err(err) = add_tag(file_path, new_register) {
            panic!("Error running add_tag: {}", err);
        }
    }
    else {
        if let Err(err) = add_sentence(file_path, new_register) {
            panic!("Error running add_tag: {}", err);
        }
    }
    Ok(())
}

pub fn link(
    sentence_file_path: String, 
    tag_file_path: String, 
    sentence_tag_file_path: String, 
    id_sentence: String,
    id_tag: String
) -> Result<(), Box<dyn Error>> {
    // check if this link already exists
    let mut sentence_tag_reader = ReaderBuilder::new().from_path(
        sentence_tag_file_path.clone())?;
    for result in sentence_tag_reader.deserialize() {
        let record: SentenceTag = result?;
        if record.id_sentence == id_sentence  && record.id_tag == id_tag {
            panic!("Link between sentence_id {} and tag_id {} already exists.",
                id_sentence, id_tag);
        }
    }
    // check if id_sentence exists
    let mut sentence_reader = ReaderBuilder::new().from_path(sentence_file_path)?;
    let mut found_sentence: bool = false;
    for result in sentence_reader.deserialize() {
        let record: Sentence = result?;
        if record.id == id_sentence {
            found_sentence = true;
            break;
        }
    }
    if !found_sentence {
        panic!("sentence_id {} does not exist.", id_sentence);
    }
    // check if id_tag exists
    let mut tag_reader = ReaderBuilder::new().from_path(tag_file_path)?;
    let mut found_tag: bool = false;
    for result in tag_reader.deserialize() {
        let record: Tag = result?;
        if record.id == id_tag {
            found_tag = true;
            break;
        }
    }
    if !found_tag {
        panic!("tag_id {} does not exist.", id_tag);
    }
    // add link
    if let Err(err) = append_record_to_file(
        sentence_tag_file_path, &[id_sentence.to_string(), id_tag.to_string()])
    {
            panic!("Error appending to file: {}", err);
    }    
    Ok(())
}
