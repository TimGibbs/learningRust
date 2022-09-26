extern crate rand;

use rand::Rng;
use std::io::{stdin,Read};
use std::path::Path;
use std::collections::HashSet;

fn get_word() -> String  {
    let path = Path::new("words.txt");
    let mut file = std::fs::File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    let words : Vec<&str> = contents.split('\n').collect();
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..852);
    let ret = words[num];
    return String::from(ret);
}

fn letter_select() -> String {
    println!("select a letter");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error: unable to read user input");
    input = input.trim().to_string();
    return input;
}

fn main() {
    let word = get_word();
    let mut display = "_".repeat(word.len() -1);
    let mut guessed = HashSet::new();
    let mut guesses : i32 = 0;
    loop {
        print!("{esc}c", esc = 27 as char);
        println!("{}", display);
        println!("{:?}",guessed);

        let input = letter_select();
        if input.len() >1 { continue };
        if input.len() ==0 { continue };

        match word.find(&input)
        {
            None => {
                guessed.insert(input);
                guesses += 1
            },
            Some(_) => {
                let v: Vec<_> = word.match_indices(&input).map(|(i, _)|i).collect();
                for i in v {
                    display.replace_range(i..i+1,&input);
                }
            },
        } 
        
        if display.find("_") == None { 
            println!("YOU WIN");
            break;
        }

        if guesses >= 11 {
            println!("YOU LOSE");
            break;
        }
    }
}