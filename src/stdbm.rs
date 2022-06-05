mod settings;
mod db_operations;
mod data_structures;

extern crate csv;

use clap::{Parser, Subcommand};
use settings::Settings;
use db_operations::{
    list, filter_by_string, filter_by_tag, //add, update, rm, save
};


const CONFIG_DATA_BLOCK: &str = "data_io.";


#[derive(Debug, Parser)]
#[clap(name = "stdbm")]
#[clap(about = "Sentences-tags database manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(
        about = "List registers of datatable",
        arg_required_else_help = true
    )]
    List {
        datatable_name: String
    },
    #[clap(
        about = "Filter and display datatable \
        registers which contain filter_string",
        arg_required_else_help = true
    )]
    FilterString {
        filter_string: String
    },
    #[clap(
        about = "Filter and display sentences \
        which are related to tag",
        arg_required_else_help = true
    )]
    FilterTag {
        tag: String
    },
    #[clap(
        about = "Add new register",
        arg_required_else_help = true
    )]
    Add {
        datatable_name: String,
        new_register: String
    },
    #[clap(
        about = "Update register value by id",
        arg_required_else_help = true
    )]
    Update {
        datatable_name: String,
        id_register: usize,
        new_value: String
    },
    #[clap(
        about = "Remove register by id",
        arg_required_else_help = true
    )]
    Rm {
        datatable_name: String,
        id_register: usize
    },
    #[clap(
        about = "Save datatable to file path",
        arg_required_else_help = true
    )]
    Save {
        datatable_name: String,
        file_path: String
    }
}


fn main() {
    let args = Cli::parse();
    let settings = Settings::new();
    let config_data_block = CONFIG_DATA_BLOCK.to_string();

    let sentence_file_path: String = settings.get_string(
        &(config_data_block.clone() + "sentences")).unwrap();
    let tag_file_path: String = settings.get_string(
        &(config_data_block.clone() + "tags")).unwrap();
    let sentence_tag_file_path: String = settings.get_string(
        &(config_data_block.clone() + "sentences_tags")).unwrap();
        

    match args.command {
        Commands::List { datatable_name } => {
            println!("Listing {}", datatable_name);
            if let Err(err) = list(
                settings.get_string(
                    &(config_data_block.clone() + &datatable_name)
                )
                .unwrap()) {
                println!("error running list {}: {}", 
                    datatable_name, err);
            }
        }
        Commands::FilterString { filter_string } => {
            println!(
                "Filtering sentences with {}", 
                    filter_string);
            if let Err(err) = filter_by_string(
                sentence_file_path,
                &filter_string
            ) {
                println!("error running filter_string: {}", 
                    err);
            }
        }
        Commands::FilterTag { tag } => {
            println!("Filtering by tag {}", tag);
            if let Err(err) = filter_by_tag(
                sentence_file_path,
                tag_file_path,
                sentence_tag_file_path,
                &tag
            ) {
                println!("error running filter_tag: {}", 
                    err);
            }
        }
        Commands::Add { datatable_name, new_register } => {
            println!(
                "Adding {} into {}", new_register, datatable_name
            );
        }
        Commands::Update { datatable_name, id_register, new_value } => {
            println!(
                "Updating {}: id {} with {}", 
                datatable_name, id_register, new_value
            );
        }
        Commands::Rm { datatable_name, id_register } => {
            println!(
                "Removing id {} from {}", 
                id_register, datatable_name 
            );
        }
        Commands::Save { datatable_name, file_path } => {
            println!(
                "Saving {} to {:?}", 
                datatable_name, file_path
            );
        }
    }
}
