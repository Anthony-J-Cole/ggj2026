use std::io::Read;
use crossterm::{self, cursor};
use rand::Rng;

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


    while true {
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Red);

        let mut rand = rand::rng();
        let mut first = rand.random_range(0..1024);
        let mut second = rand.random_range(0..1024);

        //Create the target
        let mut target = if first < second { first } else { second };
        let mut starting = if first < second { second } else { first };

        //Select a random bitmask operation
        let mut mask = match rand.random_range(0..2) {
            0 => Masks::And,
            1 => Masks::Xor,
            _ => Masks::And,
        };


        //Is it possible to reach the target from the starting number with the selected mask?
        let mut regen = false;
        while(!regen) {
            regen = true;
            match mask {
                Masks::And => {
                    //For AND, the target must have all bits set that are set in the starting number
                    if (target | starting) != starting {
                    target = rand.random_range(0..1024);
                    regen = false;
                    }
                }
                Masks::Xor => {
                //For XOR, the target must be less than or equal to twice the starting number
                if target > (starting * 2) {
                    target = rand.random_range(0..1024);
                    regen = false;
                }
            }
            }
        }


        let mut ee: EasterEggs = EasterEggs::None;

        //hehe
        if (target == starting) {
            ee = EasterEggs::SameNumber
        };
        if (target == 69 || starting == 69) {
            ee = EasterEggs::SixtyNine
        };

        println!("{}", &story_lines[story_progress]);
        story_progress += 1;

        println!(
            "Mask   =\t{}",
            match mask {
                Masks::And => "AND",
                Masks::Xor => "XOR",
            }
        );
        print_aligned_binary(target, starting, ee);
        
        //Wait for user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //Convert the input from binary string to a number
        let user_number = u32::from_str_radix(input.trim(), 2).unwrap();

        //Apply the bitmask operation
        let result = mask.apply(starting, user_number);


        //Clear the terminal
        println!("{}", crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
        println!("{}", cursor::MoveTo(0, 0));

        //Check if the result matches the target
        if result == target {
            println!("Congratulations! You matched the target.\n");
        } else {
            println!("Fail. The result was {:x}.\n", result);
        }
    }
}

fn print_aligned_binary(a: u32, b: u32, ee: EasterEggs) {
    let bin_a = format!("{:b}", a);
    let bin_b = format!("{:b}", b);

    let width = bin_a.len().max(bin_b.len());

    ee.handle();

    println!("{:0width$b} = Target Number", a, width = width);
    println!("{:0width$b} = Starting Number", b, width = width);
}

enum Masks {
    And,
    Xor,
}

impl Masks {
    fn apply(&self, target: u32, user_num: u32) -> u32 {
        match self {
            Masks::And => target & user_num,
            Masks::Xor => target ^ user_num,
        }
    }
}

enum EasterEggs {
    None,
    SameNumber,
    SixtyNine,
}

impl EasterEggs {
    fn handle(&self) {
        match self {
            EasterEggs::None => {
                println!("");
            }
            EasterEggs::SameNumber => {
                println!("Wait that wasnt supposed to happen");
            }
            EasterEggs::SixtyNine => {
                println!("Nice!");
            }
        }
    }
}

/* Gameplan,
you are a human forced to to menial labor in a world where AI has replaced everything -
Your goal is to bitmask incoming numbers to match the expected output.

Operation:








*/
