use gory::*;

fn main() {
    // Regular
    print!("{}  ", format!("\\e[1;{}m", Color::Black).black());
    print!("{}  ", format!("\\e[1;{}m", Color::Red).red());
    print!("{}  ", format!("\\e[1;{}m", Color::Green).green());
    print!("{}  ", format!("\\e[1;{}m", Color::Yellow).yellow());
    print!("{}  ", format!("\\e[1;{}m", Color::Blue).blue());
    print!("{}  ", format!("\\e[1;{}m", Color::Magenta).magenta());
    print!("{}  ", format!("\\e[1;{}m", Color::Cyan).cyan());
    print!("{}", format!("\\e[1;{}m", Color::White).white());
    println!();
}
