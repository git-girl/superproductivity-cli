use home;
use std::fs;
use serde_json::{Result, Value};

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

    // Structure Tasks have some weir hashed ids for keys
    // "Ieiivvhnb5vydGQlTH6PR": {
    //      "_showSubTasksMode":2,
    //      "attachments":[],
    //      "created":1684756335502,
    //      "doneOn":null,
    //      "id":"Ieiivvhnb5vydGQlTH6PR",
    //      "isDone":false,
    //      "issueAttachmentNr":null,
    //      "issueId":null,
    //      "issueLastUpdated":null,
    //      "issuePoints":null,
    //      "issueType":null,
    //      "issueWasUpdated":null,
    //      "notes":"",
    //      "parentId":null,
    //      "plannedAt":null,
    //      "projectId":null,
    //      "reminderId":null,
    //      "repeatCfgId":null,
    //      "subTaskIds":[],
    //      "tagIds":["TODAY"],
    //      "timeEstimate":0,
    //      "timeSpent":0,
    //      "timeSpentOnDay":{},
    //      "title":"Exercise loading the SOEP into Pandas"
    // }
}
