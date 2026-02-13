enum Type {
    Ok,
    Unclear,
    Fail,
    Error,
    Fatal,
    Location,
    Parrallel,
    Info,
    Command,
    Callback,
    Hint,
    Comment,
    Input,
    Output,
}

fn log(log_type: Type, message: &str) {
    let mut lines = message.lines();
    
    if let Some(main_log) = lines.next() {
        println!("[ {} ] {}", to_symbol(log_type), main_log);
    }

    while let Some(sub_log) = lines.next() {
        println!("  |-->> {}", sub_log);
    }
}

fn to_symbol(log_type: Type) -> char {
    match log_type {
        Type::Ok => '+',
        Type::Unclear => '?',
        Type::Fail => '-',
        Type::Error => '!',
        Type::Fatal => 'x',
        Type::Location => '@',
        Type::Parrallel => '&',
        Type::Info => 'i',
        Type::Command => '$',
        Type::Callback => '%',
        Type::Hint => '*',
        Type::Comment => ';',
        Type::Input => '<',
        Type::Output => '>',
    }
}

pub fn ok(message: &str) { log(Type::Ok, message) }
pub fn unclear(message: &str) { log(Type::Unclear, message) }
pub fn fail(message: &str) { log(Type::Fail, message) }
pub fn error(message: &str) { log(Type::Error, message) }
pub fn fatal(message: &str) { log(Type::Fatal, message) }
pub fn location(message: &str) { log(Type::Location, message) }
pub fn parrallel(message: &str) { log(Type::Parrallel, message) }
pub fn info(message: &str) { log(Type::Info, message) }
pub fn command(message: &str) { log(Type::Command, message) }
pub fn callback(message: &str) { log(Type::Callback, message) }
pub fn hint(message: &str) { log(Type::Hint, message) }
pub fn comment(message: &str) { log(Type::Comment, message) }
pub fn input(message: &str) { log(Type::Input, message) }
pub fn output(message: &str) { log(Type::Output, message) }

