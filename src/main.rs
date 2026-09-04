// Print the lyrics to The Twelve Days of Christmas in sequence
#![allow(unused)]
use std::io;
use std::{thread, time};
use std::process;

// was gonna use this but indexing was funky so I resorted to function calls
// Then this got to where I'd need to convert between u32 and usize and I didn't 
// want to do that... 
// But now I'm doing that...
const LYRICS: [&str; 12] = [
  "a partridge in a pear tree!\n",
  "two turtle doves and",
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

const FIVE_GOLDEN_RINGS: [&str; 3] = [
  "FIVE!",
  "GOLDEN!",
  "RINGS!",
];

fn main() {
  // exp();
  // quit();
  loop {
    // welcome
    welcome();
    // get user input and print lyrics
    handle_input(get_user_input());
    // sequence_lyrics(7, 1);
    // ask to repeat
    // go_again(get_user_input());
  }
}

fn welcome() {
  // clear screen
  clear_screen();
  // display welcome message
  println!("welcome to twlvi!");
  println!("please choose a number n where (0 < n < 13)");
  println!("(f: no animation)");
  println!("(ctrl-c/q: quit)");
}

fn push_all_stanzas() {
  // disable sleep for the stanza printing loop
  println!("pushing all stanzas!");
  sequence_lyrics(12, 0);
  quit();
  return
}

// touch up when you learn enums
fn handle_input(input: String) -> String {
  loop {
    // println!("not really");
    // get stanzas to print
    let mut stanzas = match input.trim().parse(){
      Ok(ret) => ret,
      Err(_) => {
        handle_input(get_user_input());
        0
      },
    };

    match input.trim() {
      "1" => sequence_lyrics(stanzas, 1),
      "2" => sequence_lyrics(stanzas, 1),
      "3" => sequence_lyrics(stanzas, 1),
      "4" => sequence_lyrics(stanzas, 1),
      "5" => sequence_lyrics(stanzas, 1),
      "6" => sequence_lyrics(stanzas, 1),
      "7" => sequence_lyrics(stanzas, 1),
      "8" => sequence_lyrics(stanzas, 1),
      "9" => sequence_lyrics(stanzas, 1),
      "10" => sequence_lyrics(stanzas, 1),
      "11" => sequence_lyrics(stanzas, 1),
      "12" => sequence_lyrics(stanzas, 1),
      "f" => push_all_stanzas(),
      "q" => quit(),
      _ => (),
    }
  }
}

// touch up when you learn enums
fn go_again(input: String) {
  loop {
    println!("Would you like to go again?");
    // let mut choice = get_user_input();

    match get_user_input().as_str().trim() {
      "y" => return,
      "" => return,
      "n" => quit(),
      _ => { 
        println!("choice invalid"); 
        continue
      },
    }
  }
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
  sleep_sec(Sec::Is(2));
}

fn five(sleep_time: u64) {
  for i in (1..=3){
    println!("{}", FIVE_GOLDEN_RINGS[i - 1]);
    sleep_sec(Sec::Is(sleep_time));
  }
}

// print lyric starting from target and ending at the front of the array
fn pluck_lyric_by_index(target: u32, sleep_time: u64) {

  // pluck lyric in sequence
  // 1
  // 2 1
  // 3 2 1
  // 4 3 2 1
  // we want to print from the array in reverse
  // the start of the printed section is i
  // we can print each element with a for loop
  // how can we only print from one index on?
  let lyrics = &LYRICS;

  let mut iter = target as usize;
  while iter > 0 {
    if iter == 5 {
      five(1);
      iter -= 1;
    }
    println!("{}", lyrics[iter - 1]);
    sleep_sec(Sec::Is(1));
    iter -= 1;
  }
  return

  sleep_sec(Sec::Is(sleep_time));
}

// arrange the corresponding gift from LYRICS
// and subsequent gifts in decending order
fn sequence_lyrics(n: u32, sleep_time: u64) {

  for i in (1..=n){
    // print verse header
    println!("[Verse {i}]");
    // print "On The" for the day
    on_the(i);
    match i {
      1 => pluck_lyric_by_index(i, sleep_time),
      2 => pluck_lyric_by_index(i, sleep_time),
      3 => pluck_lyric_by_index(i, sleep_time),
      4 => pluck_lyric_by_index(i, sleep_time),
      5 => pluck_lyric_by_index(i, sleep_time),
      6 => pluck_lyric_by_index(i, sleep_time),
      7 => pluck_lyric_by_index(i, sleep_time),
      8 => pluck_lyric_by_index(i, sleep_time),
      9 => pluck_lyric_by_index(i, sleep_time),
      10 => pluck_lyric_by_index(i, sleep_time),
      11 => pluck_lyric_by_index(i, sleep_time),
      12 => pluck_lyric_by_index(i, sleep_time),
      _ => (),
    };
  };
}

////////////////////////////
//////// HELPERS ///////////
////////////////////////////

fn clear_screen() {
  print!("\x1b[2J\x1b[1;1H");
  // another option:
  // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn get_user_input() -> String {
  // Ask user for input
  loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    return input;
  }
}

fn quit() {
  process::exit(0x100)
}

// implement optional function argument with helpful compiler output for sleep 
// function
// Don't understand enough about this to implement it now
// probably might just check if t == 1 inside function
enum Sec {
  Empty,
  Is(u64),
}

impl Default for Sec {
  fn default() -> Self {
    // want to make 1 the default argument for the parameter if none are passed
    // but don't get how
    Sec::Is(1)
    // Sec::Is(u64::from(1)) // <- no worko
  }
}

// sleep
fn sleep_sec(t: Sec) {

  // Something something implement default parameter

  let millis = match t {
    // Sec::Empty => time::Duration::from_millis(Sec::default()), // <- no worko
    Sec::Empty => time::Duration::from_millis(1 * 1000),
    Sec::Is(value) => time::Duration::from_millis(value * 1000)
  };

  // check if t with if let?
  // let millis = time::Duration::from_millis(t * 1000); 
  let now = time::Instant::now();

  thread::sleep(millis);

  assert!(now.elapsed() >= millis)
}

////////////////////////////
//////// BAD IDEAS /////////
////////////////////////////

// lyrics function chain idea
// This worked but LUL it's terrible
fn first() {
  println!("a partridge in a pair tree!\n");
  sleep_sec(Sec::Empty);
}

fn second() {
  println!("two turtle doves and");
  sleep_sec(Sec::Empty);
  first();
}

fn third() {
  println!("three french hens");
  sleep_sec(Sec::Empty);
  second();
}

fn fourth() {
  println!("four calling birds");
  sleep_sec(Sec::Empty);
  third();
}

fn fifth() {
  println!("FIVE.");
  //sleep command
  sleep_sec(Sec::Is(2));
  println!("GOLDEN.");
  //sleep command
  sleep_sec(Sec::Is(2));
  println!("RINGS.");
  //sleep command
  sleep_sec(Sec::Is(2));
  fourth();
}

fn sixth() {
  println!("six geese a-laying");
  sleep_sec(Sec::Empty);
  fifth();
}

fn seventh() {
  println!("seven swans a-swimming");
  sleep_sec(Sec::Empty);
  sixth();
}

fn eigth() {
  println!("eight maids a-milking");
  sleep_sec(Sec::Empty);
  seventh();
}

fn ninth() {
  println!("nine ladies dancing");
  sleep_sec(Sec::Empty);
  eigth();
}

fn tenth() {
  println!("ten lords a-leaping");
  sleep_sec(Sec::Empty);
  ninth();
}

fn eleventh() {
  println!("eleven pipers piping");
  sleep_sec(Sec::Empty);
  tenth();
}

fn twelfth() {
  println!("twelve drummers drumming");
  sleep_sec(Sec::Empty);
  eleventh();
}

fn old_func_delete_me(n: u32) {
  // print lyrics in sequence
  // 1
  // 2 1
  // 3 2 1
  // 4 3 2 1
  for i in (1..=n){
    // print verse header
    println!("[Verse {i}]");
    // print "On The" for the day
    on_the(i);
    // print the corresponding gift from LYRICS
    // and subsequent gifts in decending order
    // we want to print from the array in reverse
    // the start of the printed section is i
    // we can print each element with a for loop
    // how can we only print from one index on?
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
