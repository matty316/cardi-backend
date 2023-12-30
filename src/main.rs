mod project;

use crate::project::{Project, Craft, Status};
use clap::{Subcommand, Parser};
use std::fs;
use std::ffi::OsString;

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
        #[arg(short, long, required=true)]
        name: String,

        #[arg(long)]
        new_name: Option<String>,

        #[arg(short, long)]
        craft: Option<String>,
        
        #[arg(long)]
        notes: Option<String>,
        
        #[arg(short, long)]
        status: Option<String>,

        #[arg(short, long)]
        progress: Option<i32>,

        #[arg(long)]
        current_row: Option<i32>,
    },

    #[command(arg_required_else_help = true)]
    Increment {
        #[arg(short, long, required=true)]
        name: String,
    },

    #[command(arg_required_else_help = false)]
    View {
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    let args = CardiCli::parse();
    match args.command {
        Commands::New { name, craft } => create_project(&name, &craft),
        Commands::Edit { name, new_name, craft, notes, status, progress, current_row } => {
            edit_project(&name, new_name, craft, notes, status, progress, current_row)
        }
        Commands::Increment { name } => increment_row(&name),
        Commands::View { name } => view(name),
    }
}

fn get_path(name: &str) -> OsString {
    let mut path = dirs::home_dir().unwrap().into_os_string();
    let path_string = format!("/.cardi/data/{name}.json");
    path.push(path_string);
    path
}

fn fetch(name: &str) -> Project {
    let path = get_path(name);
    let json = fs::read_to_string(path.clone()).unwrap();
    serde_json::from_str::<Project>(&json).unwrap()
}

fn save(name: &str, project: Project) {
    let path = get_path(name);
    let json = serde_json::to_string(&project).unwrap();
    fs::write(path.clone(), json.clone()).expect("unable to save project");
    println!("{project:?}");
}

fn create_project(name: &str, craft: &str) {
     let craft_enum = craft_from_string(&craft);

     let project = Project::new(name.to_string(), craft_enum);
     let json = serde_json::to_string(&project).unwrap();
     let mut home_dir = dirs::home_dir().unwrap().into_os_string();
     home_dir.push("/.cardi/data");
     fs::create_dir_all(home_dir.clone()).expect("could not create data directory");
     let filename = format!("/{name}.json");
     home_dir.push(filename);
     fs::write(home_dir.clone(), json).expect("could not save project");
}

fn craft_from_string(craft: &str) -> Craft {
    match craft.to_lowercase().as_str() {
        "crochet" => Craft::Crochet,
        "knitting" => Craft::Knitting,
        "both" => Craft::Both,
        _ => {
            println!("craft can be crochet, knitting or both");
            std::process::exit(65);
        }
    }
}

fn status_from_string(status: &str) -> Status {
    match status.to_lowercase().as_str() {
        "not-started" => Status::NotStarted,
        "in-progress" => Status::InProgress,
        "finished" => Status::Finished,
        _ => {
            println!("status can be not-started, in-progress or finished");
            std::process::exit(65);
        }
    }
}

fn edit_project(name: &str,
                new_name: Option<String>,
                craft: Option<String>,
                notes: Option<String>,
                status: Option<String>,
                progress: Option<i32>,
                current_row: Option<i32>) {
    let mut project = fetch(name);
    if let Some(n) = new_name {
        if n != project.name { project.name = n.to_string(); }
    }

    if let Some(c) = craft {
        let c_enum = craft_from_string(&c);
        if c_enum != project.craft { project.craft = c_enum; }
    }

    if let Some(n) = notes {
        if n != project.notes { project.notes = n; }
    }

    if let Some(s) = status {
        let s_enum = status_from_string(&s);
        if s_enum != project.status { project.status = s_enum }
    }

    if let Some(r) = current_row {
        if r != project.current_row { project.current_row = r }
    }

    if let Some(p) = progress {
        if p < 0 || p > 100 {
            println!("progress can be between 0 and 100");
            std::process::exit(65);
        }
        if p != project.progress { project.progress = p }
    }

    save(name, project);
}

fn increment_row(name: &str) {
    let mut project = fetch(name);
    project.current_row += 1;
    save(name, project);
}

fn view(name: Option<String>) {
    if let Some(n) = name {
        let project = fetch(&n);
        println!("{project:?}");
    } else {
        let mut path = dirs::home_dir().unwrap().into_os_string();
        path.push("/.cardi/data");
        let dir = fs::read_dir(path).expect("cannot access data folder");

        for path in dir {
            let json = fs::read_to_string(path.unwrap().path().into_os_string()).unwrap();
            let project = serde_json::from_str::<Project>(&json).unwrap();
            println!("{project:?}");
        }
    }
}
