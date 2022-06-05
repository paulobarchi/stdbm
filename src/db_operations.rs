use std::error::Error;
use csv::ReaderBuilder;
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
        println!("No records with {} ", filter_string);
    }
    Ok(())
}


pub fn filter_by_tag(
    sentence_file_path: String, 
    tag_file_path: String, 
    sentence_tag_file_path: String, 
    filter_tag: &str
) -> Result<(), Box<dyn Error>> {

    let mut sentence_reader = ReaderBuilder::new().from_path(
        sentence_file_path)?;

    let mut tag_reader = ReaderBuilder::new().from_path(
        tag_file_path)?;

    let mut sentence_tag_reader = ReaderBuilder::new().from_path(
        sentence_tag_file_path)?;

    let mut found_tag: bool = false;
    let mut found_sentence: bool = false;
    let mut id_tag: String = String::new();
    let mut id_sentences: Vec<String> = Vec::new();
    
    // search for tag
    for result in tag_reader.deserialize() {
        let record: Tag = result?;
        if record.tag.contains(filter_tag) {
            found_tag = true;
            id_tag = record.id;
        }
    }
    if found_tag {
        // retrieve sentence_ids that have such tag
        for result in sentence_tag_reader.deserialize() {
             let record: SentenceTag = result?;
            if record.id_tag == id_tag {
                found_sentence = true;
                id_sentences.push(record.id_sentence);
            }
        }
        if found_sentence {
            for result in sentence_reader.deserialize() {
                let record: Sentence = result?;
                for sentence_id in &id_sentences {
                    if &record.id == sentence_id {
                        record.print();
                    }
                }
             }
        }
    }

    if !found_tag || !found_sentence {
        println!("No records with {} ", filter_tag);
    }
    Ok(())
}
