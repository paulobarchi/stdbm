use serde::{Deserialize};

pub trait Print {
    fn print(&self) -> ();
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Sentence {
    pub id: String,
    pub sentence: String
}

impl Print for Sentence {
    fn print(&self) -> () {
        println!("{:?}", self.sentence)
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Tag {
    pub id: String,
    pub tag: String
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SentenceTag {
    pub id_sentence: String,
    pub id_tag: String
}
