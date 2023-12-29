use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] 
pub struct Project {
    name: String,
    craft: Craft,
    current_row: i32,
    notes: String,
    progress: i32,
    status: Status,
    started: DateTime<Utc>
}

impl Project {
    pub fn new(name: String, craft: Craft) -> Self {
        Project {
            name,
            craft,
            current_row: 1,
            notes: "".to_string(),
            progress: 0,
            status: Status::NotStarted,
            started: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)] 
pub enum Status {
    NotStarted,
    InProgress,
    Finished
}

#[derive(Debug, Serialize, Deserialize)] 
pub enum Craft {
    Crochet,
    Knitting,
    Both
}
