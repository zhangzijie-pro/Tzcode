
#[derive(Debug, PartialEq, PartialOrd, Eq,Clone)]
pub enum CommandPriority {
    Low,
    Medium,
    High,
    Unknow,
}

impl CommandPriority{
    pub fn as_number(&self) -> u8 {
        match *self {
            CommandPriority::Unknow => 0,
            CommandPriority::Low => 1,
            CommandPriority::Medium => 2,
            CommandPriority::High => 3,
        }
    }
}

impl Ord for CommandPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_number().cmp(&other.as_number())
    }
}

// Set command priority
pub fn get_priority(command: &str) -> CommandPriority{
    match command{
        "pwd"|"ls"|"mkdir"|"touch"|"whoami"|"exit"|"echo"|"print" => CommandPriority::Low,
        "cd"|"rm"|"cat"|"python"|"html"|"web"|"rn"|"mv"|"tar"|"grep"|"pd"|"root"|"apt"|"history" => CommandPriority::Medium,
        "sleep"|"kill"|"ps"=> CommandPriority::High,
        _ => CommandPriority::Unknow
    }
}