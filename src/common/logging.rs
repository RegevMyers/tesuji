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

#[allow(unused)] pub fn ok(message: &str) { log(Type::Ok, message) }
#[allow(unused)] pub fn unclear(message: &str) { log(Type::Unclear, message) }
#[allow(unused)] pub fn fail(message: &str) { log(Type::Fail, message) }
#[allow(unused)] pub fn error(message: &str) { log(Type::Error, message) }
#[allow(unused)] pub fn fatal(message: &str) { log(Type::Fatal, message) }
#[allow(unused)] pub fn location(message: &str) { log(Type::Location, message) }
#[allow(unused)] pub fn parrallel(message: &str) { log(Type::Parrallel, message) }
#[allow(unused)] pub fn info(message: &str) { log(Type::Info, message) }
#[allow(unused)] pub fn command(message: &str) { log(Type::Command, message) }
#[allow(unused)] pub fn callback(message: &str) { log(Type::Callback, message) }
#[allow(unused)] pub fn hint(message: &str) { log(Type::Hint, message) }
#[allow(unused)] pub fn comment(message: &str) { log(Type::Comment, message) }
#[allow(unused)] pub fn input(message: &str) { log(Type::Input, message) }
#[allow(unused)] pub fn output(message: &str) { log(Type::Output, message) }

