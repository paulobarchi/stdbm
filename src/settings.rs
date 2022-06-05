use config::{Config, File};
use serde::{Deserialize};


// // Default values in case we need them in the future
// const DEFAULT_DATA_FILE_TYPE: &str = "csv";
// const DEFAULT_SENTENCES_FILE: &str = "sentences.csv"
// const DEFAULT_TAGS_FILE: &str = "tags.csv"
// const DEFAULT_SENTENCES_TAGS_FILE: &str = "sentences-tags.csv"
// const DEFAULT_CHECK_IDS_ON_LOAD: &str = false
// const DEFAULT_OVERWRITE: &str = true
// const DEFAULT_BCKP_FOLDER: &str = "bckp/"


#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub config_type: String,
    pub data_io: DataIO
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DataIO {
    pub data_file_type: String,
    pub sentences: String,
    pub tags: String,
    pub sentences_tags: String,
    pub check_ids_on_load: bool,
    pub overwrite: bool,
    pub bckp_folder: String
}


impl Settings {
    pub fn new() -> Config {
        return Config::builder()
            .add_source(File::with_name("config/default"))
            .build().unwrap();
    }
}
