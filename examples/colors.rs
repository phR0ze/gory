use gory::*;

fn main() {
    // Regular
    print!("{}  ", format!("\\e[0;{}m", Color::Black).black());
    print!("{}  ", format!("\\e[0;{}m", Color::Red).red());
    print!("{}  ", format!("\\e[0;{}m", Color::Green).green());
    print!("{}  ", format!("\\e[0;{}m", Color::Yellow).yellow());
    print!("{}  ", format!("\\e[0;{}m", Color::Blue).blue());
    print!("{}  ", format!("\\e[0;{}m", Color::Magenta).magenta());
    print!("{}  ", format!("\\e[0;{}m", Color::Cyan).cyan());
    print!("{}", format!("\\e[0;{}m", Color::White).white());
    println!();

    // Bright
    print!("{}  ", format!("\\e[0;{}m", Color::BrightBlack).bright_black());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightRed).bright_red());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightGreen).bright_green());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightYellow).bright_yellow());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightBlue).bright_blue());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightMagenta).bright_magenta());
    print!("{}  ", format!("\\e[0;{}m", Color::BrightCyan).bright_cyan());
    print!("{}", format!("\\e[0;{}m", Color::BrightWhite).bright_white());
    println!();

    // Bold Regular
    print!("{}  ", format!("\\e[1;{}m", Color::Black).bold_black());
    print!("{}  ", format!("\\e[1;{}m", Color::Red).bold_red());
    print!("{}  ", format!("\\e[1;{}m", Color::Green).bold_green());
    print!("{}  ", format!("\\e[1;{}m", Color::Yellow).bold_yellow());
    print!("{}  ", format!("\\e[1;{}m", Color::Blue).bold_blue());
    print!("{}  ", format!("\\e[1;{}m", Color::Magenta).bold_magenta());
    print!("{}  ", format!("\\e[1;{}m", Color::Cyan).bold_cyan());
    print!("{}", format!("\\e[1;{}m", Color::White).bold_white());
    println!();

    // Bold Bright
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightBlack).bright_black());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightRed).bright_red());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightGreen).bright_green());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightYellow).bright_yellow());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightBlue).bright_blue());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightMagenta).bright_magenta());
    // print!("{}  ", format!("\\e[0;{}m", Color::BrightCyan).bright_cyan());
    // print!("{}", format!("\\e[0;{}m", Color::BrightWhite).bright_white());
    // println!();
}
