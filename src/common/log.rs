use colored::{ Colorize, ColoredString };

enum Type {
    Ok, 
    Unclear, 
    Fail, 
    Error, 
    Fatal, 
    Location, 
    Parrallel, 
    Command, 
    Info, 
    Hint, 
    Trace, 
    Input, 
    Output, 
}

fn log(log_type: Type, message: &str) {
    let mut lines = message.lines();
    
    if let Some(main_log) = lines.next() {
        let symbol = get_symbol(log_type);
        println!("[ {} ] {}", symbol, main_log);
    }

    while let Some(sub_log) = lines.next() {
        println!("  |---> {}", sub_log);
    }
}

fn get_symbol(log_type: Type) -> ColoredString {
    match log_type {
        Type::Ok        => "+".green(),
        Type::Unclear   => "?".yellow(),
        Type::Fail      => "-".red(),
        Type::Error     => "!".bright_red(),
        Type::Fatal     => "X".bold().bright_red(),
        Type::Location  => "@".white(),
        Type::Parrallel => "&".white(),
        Type::Command   => "$".white(),
        Type::Info      => "i".blue(),
        Type::Hint      => "*".purple().dimmed(),
        Type::Trace     => ";".dimmed(),
        Type::Input     => "<".cyan(),
        Type::Output    => ">".cyan(),
    }
}

#[allow(unused)] pub fn ok(message: &str) { log(Type::Ok, message) }
#[allow(unused)] pub fn unclear(message: &str) { log(Type::Unclear, message) }
#[allow(unused)] pub fn fail(message: &str) { log(Type::Fail, message) }
#[allow(unused)] pub fn error(message: &str) { log(Type::Error, message) }
#[allow(unused)] pub fn fatal(message: &str) { log(Type::Fatal, message) }
#[allow(unused)] pub fn location(message: &str) { log(Type::Location, message) }
#[allow(unused)] pub fn parrallel(message: &str) { log(Type::Parrallel, message) }
#[allow(unused)] pub fn command(message: &str) { log(Type::Command, message) }
#[allow(unused)] pub fn info(message: &str) { log(Type::Info, message) }
#[allow(unused)] pub fn hint(message: &str) { log(Type::Hint, message) }
#[allow(unused)] pub fn comment(message: &str) { log(Type::Trace, message) }
#[allow(unused)] pub fn input(message: &str) { log(Type::Input, message) }
#[allow(unused)] pub fn output(message: &str) { log(Type::Output, message) }

