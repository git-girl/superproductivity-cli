use super::collections::{
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

pub struct Improvement { 
    checked_days: Vec<i16>,
    id: String, 
    is_repeat: bool, 
    title: String,
}
pub struct Metric { 
    id: String,
    improvements: ImprovementCollection,
    improvementsTomorrow: ImprovementCollection,
    mood: u8,
    obstructions: ObstructionCollection,
    productivity: u8,
}
pub struct Note { 
}
pub struct Obstruction { 
}
pub struct Project { 
}
pub struct Reminder { 
}
pub struct SimpleCounter { 
}
pub struct Tag { 
}
pub struct Task { 
}
pub struct TaskArchive { 
}
pub struct TaskRepeatCfg { 
}
