mod project;

use crate::project::{Project, Craft};
use clap::{Subcommand, Parser};
use std::fs;

#[derive(Parser, Debug)]
#[command(name="cardi")]
#[command(about="a pointless cli for tracking knitting and crochet projects", long_about=None)]
struct CardiCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    New {
        #[arg(short, long, required=true)]
        name: String,

        #[arg(short, long, required=true)]
        craft: String,
    },

    #[command(arg_required_else_help = true)]
    Edit {
        #[arg(short, long)]
        name: String,
        
        #[arg(short, long)]
        craft: String,
        
        #[arg(short, long)]
        notes: String,
        
        #[arg(short, long)]
        status: String,

        #[arg(short, long)]
        progress: i32,

        #[arg(short, long)]
        current_row: i32,
    }
}

fn main() {
    let args = CardiCli::parse();
    match args.command {
        Commands::New { name, craft } => create_project(&name, &craft),
        Commands::Edit { name, craft, notes, status, progress, current_row } => {
            edit_project(name, craft, notes, status, progress, current_row)
        }
    }
}

fn create_project(name: &str, craft: &str) {
     if !validate_craft(craft) {
         eprintln!("craft can be crochet, knitting or both");
         std::process::exit(65);
     }

     let craft_enum = craft_enum_from_string(&craft);

     let project = Project::new(name.to_string(), craft_enum);
     let json = serde_json::to_string(&project).unwrap();
     let mut home_dir = dirs::home_dir().unwrap().into_os_string();
     home_dir.push("/.cardi/data");
     fs::create_dir_all(home_dir.clone()).expect("could not create data directory");
     let filename = format!("/{name}.json");
     home_dir.push(filename);
     fs::write(home_dir.clone(), json).expect("could not save project");
}

fn validate_craft(craft: &str) -> bool {
    craft.to_lowercase() == "crochet" ||
        craft.to_lowercase() == "knitting" ||
        craft.to_lowercase() == "both"
}

fn craft_enum_from_string(craft: &str) -> Craft {
    match craft.to_lowercase().as_str() {
        "crochet" => Craft::Crochet,
        "knitting" => Craft::Knitting,
        "both" => Craft::Both,
        _ => panic!("should not get here"),
    }
}

fn edit_project(name: &str, craft: &str, notes: &str, status: &str, progress: i32, current_row: i32) {

}
