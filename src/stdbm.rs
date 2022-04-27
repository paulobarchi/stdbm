mod settings;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use settings::Settings;


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
        about = "Start (load) DB with info from config file", 
        arg_required_else_help = true
    )]
    Start { },
    #[clap(
        about = "Load csv file to manipulate data",
        arg_required_else_help = true
    )]
    Load {
        datatable_name: String,
        #[clap(required = true, parse(from_os_str))]
        file_path: Vec<PathBuf>,
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
    Filter {
        datatable_name: String,
        filter_string: String
    },
    #[clap(
        about = "Update register value by id",
        arg_required_else_help = true
    )]
    Update {
        datatable_name: String,
        id_register: usize,
        new_value: String,
    },
    #[clap(
        about = "Remove register by id",
        arg_required_else_help = true
    )]
    Rm {
        datatable_name: String,
        id_register: usize,
    },
    #[clap(
        about = "Save datatable to file path",
        arg_required_else_help = true
    )]
    Save {
        datatable_name: String,
        #[clap(required = true, parse(from_os_str))]
        file_path: Vec<PathBuf>,
    }
}


fn main() {
    let settings = Settings::new();
    let args = Cli::parse();

    match args.command {
        Commands::Start { } => {
            println!("Starting with settings from config file:");
            println!("{:?}", settings);
        }
        Commands::Load { datatable_name, file_path } => {
            println!(
                "Loading {:?} as {}", file_path, datatable_name
            );
        }
        Commands::Add { datatable_name, new_register } => {
            println!(
                "Adding {} into {}", new_register, datatable_name
            );
        }
        Commands::List { datatable_name } => {
            println!("Listing {}", datatable_name);
        }
        Commands::Filter { datatable_name, filter_string } => {
            println!(
                "Filtering {} with {}", datatable_name, filter_string
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
