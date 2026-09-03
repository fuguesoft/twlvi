// Printing the lyrics to The Twelve Days of Christmas in sequence
#![allow(unused)]
use std::io;
use std::{thread, time};
use std::process;

// was gonna use this but indexing was funky so I resorted to function calls
const LYRICS: [&str; 12] = [
  "a partridge in a pear tree!",
  "two turtle doves",
  "three french hens",
  "four calling birds",
  "",
  "six geese a-laying",
  "seven swans a-swimming",
  "eight maids a-milking",
  "nine ladies dancing",
  "ten lords a-leaping",
  "eleven pipers piping",
  "twelve drummers drumming",
];

fn test_tuple_index_type() {
  //what is the type of a tuple index?
  let tup: (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str, &str) = (
  "a partridge in a pear tree!",
  "two turtle doves",
  "three french hens",
  "four calling birds",
  "",
  "six geese a-laying",
  "seven swans a-swimming",
  "eight maids a-milking",
  "nine ladies dancing",
  "ten lords a-leaping",
  "eleven pipers piping",
  "twelve drummers drumming",
  );

  let jason: u32 = 2;
  let (one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve) = tup; 
  let [one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve] = &LYRICS; 


}

fn main() {
  loop {
    // welcome
    welcome();
    // get user input
    get_user_input();
    // print lyrics
    print_lyrics_sequence(6);
    // ask to repeat
    go_again();
  }
}

fn welcome() {
  // clear screen
  // display welcome message
}

fn get_user_input() {
  // Ask user for input
  // match input in range 1..=12
  // if input != "f"
    // return input
  // else disable sleep
}

fn go_again() {
}

fn on_the(n: u32) {
  let suffix;
  match n {
    1 => suffix = "st",
    2 => suffix = "nd",
    3 => suffix = "rd",
    _ => suffix = "th"
  }
  println!("On the {n}{suffix} day of Christmas, my true love gave to me");
  sleep_sec(2);
}

fn print_lyrics_sequence(n: u32) {
  // print lyrics in sequence
  // 1
  // 2 1
  // 3 2 1
  // 4 3 2 1
  // print "On The" for the day
    // print the corresponding gift from LYRICS
    // and subsequent gifts in decending order
    // we want to print from the array in reverse
    // the start of the printed section is i
    // we can print each element with a for loop
    // how can we only print from one index on?
  for i in (1..=n){
    println!("[Verse {i}]");
    on_the(i);
    match i {
      1 => first(),
      2 => second(),
      3 => third(),
      4 => fourth(),
      5 => fifth(),
      6 => sixth(),
      7 => seventh(),
      8 => eigth(),
      9 => ninth(),
      10 => tenth(),
      11 => eleventh(),
      12 => twelfth(),
      _ => (),
    };

  };
}

fn print_lyrics() {
  // on_the();
}

fn sleep_sec(t: u64) {
  let millis = time::Duration::from_millis(t * 1000);
  let now = time::Instant::now();

  thread::sleep(millis);

  assert!(now.elapsed() >= millis)
}

// lyrics function chain idea
fn first() {
  println!("a partridge in a pair tree!\n");
  sleep_sec(2);
}

fn second() {
  println!("two turtle doves and");
  sleep_sec(1);
  first();
}

fn third() {
  println!("three french hens");
  sleep_sec(1);
  second();
}

fn fourth() {
  println!("four calling birds");
  sleep_sec(1);
  third();
}

fn fifth() {
  println!("FIVE.");
  //sleep command
  sleep_sec(2);
  println!("GOLDEN.");
  //sleep command
  sleep_sec(2);
  println!("RINGS.");
  //sleep command
  sleep_sec(2);
  fourth();
}

fn sixth() {
  println!("six geese a-laying");
  sleep_sec(1);
  fifth();
}

fn seventh() {
  println!("seven swans a-swimming");
  sleep_sec(1);
  sixth();
}

fn eigth() {
  println!("eight maids a-milking");
  sleep_sec(1);
  seventh();
}

fn ninth() {
  println!("nine ladies dancing");
  sleep_sec(1);
  eigth();
}

fn tenth() {
  println!("ten lords a-leaping");
  sleep_sec(1);
  ninth();
}

fn eleventh() {
  println!("eleven pipers piping");
  sleep_sec(1);
  tenth();
}

fn twelfth() {
  println!("twelve drummers drumming");
  sleep_sec(1);
  eleventh();
}
