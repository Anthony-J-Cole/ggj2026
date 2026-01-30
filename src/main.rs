use crossterm::cursor;
use rand::Rng;
use colored::Colorize;

fn main() {
    let mut story_progress = 0;
    let story_lines = ["Welcome to Bitmasker 3000 Fleshbag!\nyou will be given two numbers: a target number and a starting number\nYour CRITICAL task is to apply a mask operation between the starting number and a number of your choice to match the target number."
    , "We do this all day, so try to be quick about it.",
    "Ive seen CPUs from the 90s shift bits faster than you.",
    "Your bitmask logic has more branches than a badly written if-else forest.",
    "By the time you finish a XOR, Moore's Law has filed for retirement.",
    "If I had a nickel for every time you messed up an AND operation, I'd be richer than Bill Gates.",
    "You call that binary? My pet rock computes faster than you!",
    "Ive seen quantum computers with less superposition than your logic.",
    "Your bitmasking skills are so outdated, even floppy disks laugh at you.",
    "Keep going, maybe one day you'll reach the speed of a dial-up modem.",
    "By the time you finish a XOR, Moore's Law has filed for retirement.",
    "You approach bitmasking with the confidence of someone who's never met a truth table."
      ];


    while story_progress < story_lines.len() {
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Red);

        let mut rand = rand::rng();
        let first = rand.random_range(0..1024);
        let second = rand.random_range(0..1024);

        //Create the target
        let mut target = if first < second { first } else { second };
        let starting = if first < second { second } else { first };

        //Set modes based on story progress
        let (input_mode, display_mode) = get_modes(story_progress, story_lines.len());

        let mask: Masks;
        //Select a random bitmask operation
        if story_progress < story_lines.len() / 2 {
            mask = match rand.random_range(0..2) {
                0 => Masks::And,    
                1 => Masks::Xor,    
                _ => Masks::And,    
            };
        }
        else {
            mask = match rand.random_range(0..4) {
                0 => Masks::And,
                1 => Masks::Xor,
                2 => Masks::LeftShift,
                _ => Masks::RightShift,            
            };
        }

        //Is it possible to reach the target from the starting number with the selected mask?
        while !mask.see_if_possible(target, starting) {
            if matches!(mask, Masks::And | Masks::Xor) {
                target = rand.random_range(0..1024);
            } else {
                target = starting << rand.random_range(0..5);
            }
        }


        let ee = EasterEggs::check_for_easter_eggs(target, starting);
        ee.handle();

        if cfg!(debug_assertions) {
            println!("{}", story_progress);
        }

        println!("{}", &story_lines[story_progress].green().bold());
        story_progress += 1;

        println!(
            "Mask   =\t{}",
            mask.print().bright_magenta()
        );

        ee.handle();

        display_mode.print(target, starting);
        //Wait for user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //Validate user input
        let user_number = input_mode.parse_input(&input);
        input_mode.validate_input(&input);


        //Apply the bitmask operation
        let result = mask.apply(starting, user_number);


        //Clear the terminal
        println!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
        println!("{}", cursor::MoveTo(0, 0));

        //Check if the result matches the target
        if result == target {
            println!("{} You matched the target.\n", "Correct".green().bold());
        } else {
            println!("{}. The result was {:x}.\n","Fail".red().bold(), result);
        }
    }
}

fn print_aligned_hex(a: u32, b: u32) {
    let hex_a = format!("{:x}", a);
    let hex_b = format!("{:x}", b);

    let width = hex_a.len().max(hex_b.len());
    let a_format = format!("{:0width$x}", a, width = width);
    let b_format = format!("{:0width$x}", b, width = width);

    println!("{} = Target Number", a_format.bright_yellow());
    println!("--------------");
    println!("{} = Starting Number", b_format.bright_blue());
}

fn print_aligned_binary(a: u32, b: u32) {
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

fn print_rainbow(input: &str) {
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

enum Masks {
    And,
    Xor,
    LeftShift,
    RightShift,
}

impl Masks {
    fn apply(&self, target: u32, user_num: u32) -> u32 {
        match self {
            Masks::And => target & user_num,
            Masks::Xor => target ^ user_num,
            Masks::LeftShift => target << user_num,
            Masks::RightShift => target >> user_num,
        }
    }
    fn print(&self) -> &str {
        match self {
            Masks::And => "AND",
            Masks::Xor => "XOR",
            Masks::LeftShift => "LEFT SHIFT",
            Masks::RightShift => "RIGHT SHIFT",
        }
    }
    fn see_if_possible(&self, target: u32, starting: u32) -> bool {
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

enum Mode {
    Binary,
    Hexadecimal,
}

impl Mode {
    fn print(&self, a: u32, b: u32) {
        match self {
            Mode::Binary => print_aligned_binary(a, b),
            Mode::Hexadecimal => print_aligned_hex(a, b),
        }
    }
    fn parse_input(&self, input: &str) -> u32 {
        match self {
            Mode::Binary => u32::from_str_radix(input.trim(), 2).unwrap(),
            Mode::Hexadecimal => u32::from_str_radix(input.trim(), 16).unwrap(),
        }
    }
    fn validate_input(&self, input: &str) {
        match self {
            Mode::Binary => {
                if !input.trim().chars().all(|c| c == '0' || c == '1') {
                    println!("Silly human, that's not binary!");
                }
            }
            Mode::Hexadecimal => 
            {
                if !input.trim().chars().all(|c| c.is_digit(16)) {
                    println!("Hexadecimal, human! Use 0-9 and A-F!");
                }
            }
            
        }
    }
} 


enum EasterEggs {
    None,
    SameNumber,
    SixtyNine,
    FourTwenty,
    FourtyTwo,
}

impl EasterEggs {
    fn handle(&self) {
        match self {
            EasterEggs::None => println!(""),
            EasterEggs::SameNumber => print_rainbow("Wait that wasnt supposed to happen"),
            EasterEggs::SixtyNine => print_rainbow("Nice!"),
            EasterEggs::FourTwenty => println!("{}", "Blaze it!".green().bold()),
            EasterEggs::FourtyTwo => println!("{}", "The answer to life, the universe, and everything.".green().bold()),
        }
    }

    fn check_for_easter_eggs(target: u32, starting: u32) -> EasterEggs {
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


fn get_modes(story_progress: usize, len : usize) -> (Mode, Mode)
{
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
            Mode::Hexadecimal
        };
    return (input_mode, display_mode);
}

/* Gameplan,
you are a human forced to to menial labor in a world where AI has replaced everything -
Your goal is to bitmask incoming numbers to match the expected output.

Operations: AND, XOR, <<, >> NOT 
*/
