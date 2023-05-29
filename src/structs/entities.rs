// TODO: get a better thing for setting times 
// like define their type based on a calculation of possible values

pub enum SPType {
    Task,
}
use super::collections::{
    ImprovementCollection,
    ObstructionCollection,
  }; 

pub struct Improvement { 
    checked_days: Vec<i16>,
    id: String, 
    is_repeat: bool, 
    title: String,
}
pub struct Metric { 
    id: String,
    improvements: ImprovementCollection,
    improvements_tomorrow: ImprovementCollection,
    mood: u8,
    obstructions: ObstructionCollection,
    productivity: u8,
}
pub struct Note { 
    content: String,
    created: u16, 
    id: String ,
    is_pinned_to_today: bool,
    modified: u16, 
    project_id: String,
}
pub struct Obstruction { 
    id: String, 
    title: String
}
// WARNING: doesnt implmenet most attrs
pub struct Project { 
    id: String,
    title: String,
    note_ids: Vec<String>,
    task_ids: Vec<String>,
    // TODO: add work End and Workstart via a key value TimeCollection
}
pub struct Reminder { 
    id: String,
    related_id: String, 
    remind_at: u16,
    title: String, 
    // TODO: get a better name
    sp_type: SPType
}
// pub struct SimpleCounter { 
  // countOnDay: {},
  // isEnabled: bool,
  // isOn: bool,
  // title: String,
     // typ: String
// }

// WARNING: doesnt implmenet most attrs 
// TODO: add work end start attrs 
pub struct Tag { 
    title: String, 
    id: String, 
    task_ids: Vec<String>, 
    created: u16,
}

pub struct Task { 

 // _showSubTasksMode": 2,
 // attachments are actual file attachments
 attachments: Vec<String>,
 created: u16,
 // TODO: implement done_on might be nil might be u16 
 // maybe wrap in an option thing or something like that 
 // done_on: u16,
 id: String,
 is_done: bool,
 // issueAttachmentNr": null,
 // issueId": null,
 // issueLastUpdated": null,
 // issuePoints": null,
 // issueType": null,
 // issueWasUpdaii": null,
 notes: String,
 parent_id: String,
 planned_at: u16,
 project_id: String,
 reminder_id: String,
 repeat_cfg_id: String,
 sub_task_ids: Vec<String>,
 tag_ids: Vec<String>,
 time_estimate: u32,
 time_spent: u32,
 // timeSpentOnDay: {},
 title: String,

}

// is the same data to a Task
// pub struct TaskArchive { 
// }

pub struct TaskRepeatCfg {  
    // take the monday .. sunday thing and put it in a truth table
    days: [bool; 7], 
    default_estimate: u32, 
    id: String, 
    is_paused: bool, 
    last_task_creation: u16, 
    notes: String, 
    order: u16, 
    project_id: String, 
    // NOTE: technically an Enum again
    quick_setting: String, 
    repeat_every: u8, 
    // NOTE: technically an Enum again
    repeat_cycle: String,
    // NOTE: technically an Enum again
    remind_at: String,
    // lol wtf why is everything so weird in this data
    start_day: String, 
    start_time: String, 
    tag_ids: Vec<String>, 
    title: Sring,
}
