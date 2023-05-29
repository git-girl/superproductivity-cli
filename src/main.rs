// TODO: if possible move this into the structs folder like to have 
// a use statement for entities and collections and they import all their components
mod structs {
    pub mod entities;
    pub mod collections;
}

use structs::entities::{
    Improvement,
    Metric,
    Note,
    Obstruction,
    Project,
    Reminder,
    SimpleCounter,
    Tag,
    Task,
    TaskArchive,
    TaskRepeatCfg
}; 

use structs::collections::{
    ImprovementCollection,
    MetricCollection,
    NoteCollection,
    ObstructionCollection,
    ProjectCollection,
    ReminderCollection,
    SimpleCounterCollection,
    TagCollection,
    TaskCollection,
    TaskArchiveCollection,
    TaskRepeatCfgCollection
}; 
use home;
use std::fs;
use serde_json::Value;

struct BaseJSONObj {
    // bookmark: ,
    // globalConfig: ,
    improvement: ImprovementCollection,
    // lastLocalSyncModelChange: ,
    metric: MetricCollection,
    note: NoteCollection,
    obstruction: ObstructionCollection,
    project: ProjectCollection,
    reminders: ReminderCollection,
    simple_counter: SimpleCounterCollection,
    tag: TagCollection,
    task: TaskCollection,
    task_archive: TaskArchiveCollection,
    task_repeat_cfg: TaskRepeatCfgCollection,
}

impl BaseJSONObj {
    fn new(
        improvement: ImprovementCollection,
        metric: MetricCollection,
        note: NoteCollection,
        obstruction: ObstructionCollection,
        project: ProjectCollection,
        reminders: ReminderCollection,
        simple_counter: SimpleCounterCollection,
        tag: TagCollection,
        task: TaskCollection,
        task_archive: TaskArchiveCollection,
        task_repeat_cfg: TaskRepeatCfgCollection,
        ) -> Self {
        Self {
            improvement,
            metric,
            note,
            obstruction,
            project,
            reminders,
            simple_counter,
            tag,
            task,
            task_archive,
            task_repeat_cfg,
        }
    }
}

fn get_superprod_json(path: &str) -> Value {
    let raw_file_data = fs::read_to_string(path)
        .expect("Couldn't read superprod sync file");

    let superprod_json: Value = serde_json::from_str(&raw_file_data)
        .expect("Couldn't parse raw superprod data file to json");
    superprod_json
}

// with all sync methods you can set them up in such a manner
// that they all have the json file in some path
fn get_superprod_data_path() -> String { 
    let binding = home::home_dir()
        .expect("Couldn't get a home_directory make sure $HOME is set.");

    let home_path = binding
        .to_str()
        .expect("Error in get_superprod_data_path");

    // TODO: move to some config file or something
    home_path.to_owned() + "/Nextcloud/test.json"
}

fn main() {
    let path = get_superprod_data_path();
    // let path_as_string = &path.to_str().expect("Something went wrong...");
    let superprod_json = get_superprod_json(&path);

    println!("{}", superprod_json);

}
