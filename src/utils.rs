pub fn get_modes(story_progress: usize, len : usize) -> (Mode, Mode){
    let input_mode = if story_progress < len / 2 {
            Mode::Binary
        } else {
            let rn = rand::random_range(0..2);
            if rn == 0 {
                Mode::Binary
            } else {
                Mode::Hexadecimal
            }
        };
        let display_mode = if story_progress < len / 2 {
            Mode::Binary
        } else {
            let rn = rand::random_range(0..2);
            if rn == 0 {
                Mode::Binary
            } else {
                Mode::Hexadecimal
            }
        };
    return (input_mode, display_mode);
}

pub fn print_aligned_hex(a: u32, b: u32) {
    use colored::Colorize;
    let hex_a = format!("{:x}", a);
    let hex_b = format!("{:x}", b);

    let width = hex_a.len().max(hex_b.len());
    let a_format = format!("{:0width$x}", a, width = width);
    let b_format = format!("{:0width$x}", b, width = width);

    println!("{} = Target Number", a_format.bright_yellow());
    println!("--------------");
    println!("{} = Starting Number", b_format.bright_blue());
}

pub fn print_aligned_binary(a: u32, b: u32) {
    use colored::Colorize;
    let bin_a = format!("{:b}", a);
    let bin_b = format!("{:b}", b);

    let width = bin_a.len().max(bin_b.len());
    let a_format =format!("{:0width$b}", a, width = width);
    let b_format =format!("{:0width$b}", b, width = width);

    println!("{} = Target Number", a_format.bright_yellow());
    println!("--------------");
    println!("{} = Starting Number", b_format.bright_blue());
}

pub fn print_rainbow(input: &str) {
    use colored::Colorize;
    let colors = [
        |s: &str| s.red(),
        |s: &str| s.yellow(),
        |s: &str| s.green(),
        |s: &str| s.cyan(),
        |s: &str| s.blue(),
        |s: &str| s.magenta(),
    ];

    for (i, ch) in input.chars().enumerate() {
        let color = &colors[i % colors.len()];
        print!("{}", color(&ch.to_string()));
    }
    println!();
}

pub enum Masks {
    And,
    Xor,
    LeftShift,
    RightShift,
}

impl Masks {
    pub fn apply(&self, target: u32, user_num: u32) -> u32 {
        match self {
            Masks::And => target & user_num,
            Masks::Xor => target ^ user_num,
            Masks::LeftShift => target << user_num,
            Masks::RightShift => target >> user_num,
        }
    }
    pub fn print(&self) -> &str {
        match self {
            Masks::And => "AND",
            Masks::Xor => "XOR",
            Masks::LeftShift => "LEFT SHIFT",
            Masks::RightShift => "RIGHT SHIFT",
        }
    }
    pub fn see_if_possible(&self, target: u32, starting: u32) -> bool {
        match self {
                Masks::And => {
                    //For AND, the target must have all bits set that are set in the starting number
                    if (target | starting) != starting {
                        return false;
                    } else {
                        return true;
                    }
                }
                Masks::Xor => {
                    //For XOR, any target is possible
                    return true;
                }
                Masks::LeftShift => 
                {
                    //For left shift the target must be the same as starting shifted left by some amount
                    let mut shifted = starting;
                    while shifted < target {
                        shifted <<= 1;
                        if shifted == target {
                            return true;
                        }
                    }
                    return false;
                }
                Masks::RightShift => 
                {
                    //For right shift the target must be the same as starting shifted right by some amount
                    let mut shifted = starting;
                    while shifted > target {
                        shifted >>= 1;
                        if shifted == target {
                            return true;
                        }
                    }
                    return false;
                }
            }
        }
}

pub enum Mode {
    Binary,
    Hexadecimal,
}

impl Mode {
    pub fn to_string(&self) -> String{
        match self {
            Mode::Binary => String::from("Binary"),
            Mode::Hexadecimal => String::from("Hexadecimal"),
        }
    }

    pub fn print(&self, a: u32, b: u32) {
        match self {
            Mode::Binary => print_aligned_binary(a, b),
            Mode::Hexadecimal => print_aligned_hex(a, b),
        }
    }
    pub fn parse_input(&self, input: &str) -> u32 {
        match self {
            Mode::Binary => u32::from_str_radix(input.trim(), 2).unwrap(),
            Mode::Hexadecimal => u32::from_str_radix(input.trim(), 16).unwrap(),
        }
    }
    pub fn validate_input(&self, input: &str) -> bool {
        match self {
            Mode::Binary => {
                if !input.trim().chars().all(|c| c == '0' || c == '1') {
                    println!("Silly human, that's not binary!");
                    return false;
                }
                return true;
            }
            Mode::Hexadecimal => 
            {
                if !input.trim().chars().all(|c| c.is_digit(16)) {
                    println!("Hexadecimal, human! Use 0-9 and A-F!");
                    return false;
                }
                return true;
            }
            
        }
    }
} 


pub enum EasterEggs {
    None,
    SameNumber,
    SixtyNine,
    FourTwenty,
    FourtyTwo,
}

impl EasterEggs {
    pub fn handle(&self) {
        use colored::Colorize;
        match self {
            EasterEggs::None => println!(""),
            EasterEggs::SameNumber => print_rainbow("Wait that wasnt supposed to happen"),
            EasterEggs::SixtyNine => print_rainbow("Nice!"),
            EasterEggs::FourTwenty => println!("{}", "Blaze it!".green().bold()),
            EasterEggs::FourtyTwo => println!("{}", "The answer to life, the universe, and everything.".green().bold()),
        }
    }

    pub fn check_for_easter_eggs(target: u32, starting: u32) -> EasterEggs {
        if target == starting {
            return EasterEggs::SameNumber;
        }
        if target == 69 || starting == 69 {
            return EasterEggs::SixtyNine;
        }
        if target == 420 || starting == 420 {
            return EasterEggs::FourTwenty;
        }
        if target == 42 || starting == 42 {
            return EasterEggs::FourtyTwo;
        }
        return EasterEggs::None;
    }
}