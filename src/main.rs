use std::io::Write;

use colored::Colorize;
use crossterm::cursor;
use rand::Rng;

//smod Utils;
mod story_manger;
mod utils;

fn main() {
    let mut sm = story_manger::setup_story();
    let mut fails = 0;

    while sm.story_progress < sm.story_lines.len() {
        //Begin the game
        crossterm::terminal::SetTitle("Bitmask");
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Red);

        let mut rand = rand::rng();
        let first = rand.random_range(0..1024);
        let second = rand.random_range(0..1024);

        //Create the target
        let mut target = if first < second { first } else { second };
        let starting = if first < second { second } else { first };

        //Set modes based on story progress
        let (input_mode, display_mode) = utils::Mode::get_modes(sm.story_progress, sm.story_lines.len());

        let mask: utils::Masks;
        //Select a random bitmask operation
        if sm.story_progress < sm.story_lines.len() / 2 {
            mask = match rand.random_range(0..2) {
                0 => utils::Masks::And,
                1 => utils::Masks::Xor,
                _ => utils::Masks::And,
            };
        } else {
            //Getting to the endgame
            mask = match rand.random_range(0..3) {
                0 => utils::Masks::And,
                1 => utils::Masks::Xor,
                2 => utils::Masks::LeftShift,
                _ => utils::Masks::RightShift,
            };
        }

        //Is it possible to reach the target from the starting number with the selected mask?
        let mut attempts = 0;
        while !mask.see_if_possible(target, starting) && attempts < 10 {
            if matches!(mask, utils::Masks::And | utils::Masks::Xor) {
                target = rand.random_range(0..1024);
            } else if matches!(mask, utils::Masks::LeftShift) {
                // For left shift, generate target by shifting starting left by 0-4
                let shift_amount = rand.random_range(0..5);
                target = starting.wrapping_shl(shift_amount as u32);
                // Ensure target stays within reasonable bounds
                if target > 1048576 {
                    target = starting << rand.random_range(0..3);
                }
            } else {
                // For right shift, generate target by shifting starting right by 1-4
                let shift_amount = rand.random_range(1..5);
                target = starting >> shift_amount;
                // Avoid target being 0 unless starting is also very small
                if target == 0 && starting > 0 {
                    target = starting >> 1;
                }
            }
            attempts += 1;
        }
        sm.EasterEggs = utils::EasterEggs::check_for_easter_eggs(target, starting);

        // Round counter while debugging
        if cfg!(debug_assertions) {
            println!("{}", sm.story_progress);
        }

        //Print Storyline
        println!("{}", sm.story_lines[sm.story_progress].green().bold());
        sm.story_progress += 1;

        //Print the operation to do
        println!(
            "Mask =\t{}\tInput Mode =\t{}",
            mask.print().bright_magenta(),
            input_mode.to_string(),
        );

        //Display the target and starting numbers
        display_mode.print(target, starting);

        print!("{}", if matches!(input_mode ,utils::Mode::Binary) {"0b".purple()} else {"0x".bright_cyan()} );
        std::io::stdout().flush().expect("Failed to flusg stdout");

        //Wait for user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //Clear the terminal
        println!(
            "{}",
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        );
        println!("{}", cursor::MoveTo(0, 0));

        //Validate user input
        let valid = input_mode.validate_input(&input);
        if !valid {
            fails += 1;
            continue;
        }
        let user_number = input_mode.parse_input(&input);

        //Apply the bitmask operation
        let result = mask.apply(starting, user_number);

        let res_out = match display_mode {
            utils::Mode::Binary => format!("{:b}", result),
            utils::Mode::Hexadecimal => format!("{:x}", result),
        };

        //Check if the result matches the target
        if result == target {
            println!("{} You matched the target.", "Correct".green().bold());
        } else {
            println!("{}. The your input was {}. :(", "Fail".red().bold(), res_out);
            fails += 1;
        }
    }

    let fail_string = format!("{}", fails);
    println!(
        "{}{}{}",
        "You failed ".green(),
        if fails > 0 {
            fail_string.red()
        } else {
            fail_string.green()
        },
        " times".green()
    )
}

/* Gameplan,
you are a human forced to to menial labor in a world where AI has replaced everything -
Your goal is to bitmask incoming numbers to match the expected output.

Operations: AND, XOR, <<, >> NOT
*/
