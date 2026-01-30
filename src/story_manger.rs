pub struct Story {
  pub story_progress : usize,
  pub story_lines: [String; 12],
}


pub fn setup_story() -> Story
{
  let story_manger: Story = Story { 
    story_progress: 0,
    story_lines: [
      String::from("Welcome to Bitmasker 3000 Fleshbag!\nyou will be given two numbers: a target number and a starting number\nYour CRITICAL task is to apply a mask operation between the starting number and a number of your choice to match the target number."),
      String::from("We do this all day, so try to be quick about it."),
      String::from("Your bitmask logic has more branches than a badly written if-else forest."),
      String::from("By the time you finish a XOR, Moore's Law has filed for retirement."),
      String::from("If I had a nickel for every time you messed up an AND operation, I'd be richer than Bill Gates."),
      String::from("You call that binary? My pet rock computes faster than you!"),
      String::from("Ive seen quantum computers with less superposition than your logic."),
      String::from("Your bitmasking skills are so outdated, even floppy disks laugh at you."),
      String::from("You approach bitmasking with the confidence of someone who's never met a truth table."),
      String::from("By the time you finish a XOR, Moore's Law has filed for retirement."),
      String::from("Keep going, maybe one day you'll reach the speed of a dial-up modem."),
      String::from("Ive seen CPUs from the 90s shift bits faster than you."),
      ]
  };
  return story_manger;
}

