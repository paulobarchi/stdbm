use config::{Config, ConfigError, File};
use serde::{Deserialize};


// // Default values in case we need them in the future
// const DEFAULT_DATA_FILE_TYPE = "csv";
// const DEFAULT_SENTENCES_FILE = "sentences.csv"
// const DEFAULT_TAGS_FILE = "tags.csv"
// const DEFAULT_SENTENCES_TAGS_FILE = "sentences-tags.csv"
// const DEFAULT_CHECK_IDS_ON_LOAD = false
// const DEFAULT_OVERWRITE = true
// const DEFAULT_BCKP_FOLDER = "bckp/"


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
    pub sentences_file: String,
    pub tags_file: String,
    pub sentences_tags_file: String,
    pub check_ids_on_load: bool,
    pub overwrite: bool,
    pub bckp_folder: String
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Add "default" configuration file
            .add_source(File::with_name("config/default"))
            .build()?;
        // Deserialize (and thus freeze) the entire configuration
        config.try_deserialize()
    }
}
