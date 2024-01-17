mod word_counter;
use crate::word_counter::{WordCounter, WordActions};
use std::io;

fn main() {
    //Message to the user to type something
    println!("Please input the word");
    //a variable to assign the user's input
    let mut word = String::new();

    //reading user's input
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");//in case of an error, this message will be shown

    //taking an instance of WordCounter struct
    let new_instance = WordCounter::new(&word);
    //Word count of the given text
    println!("\n\n\nCount of word: {}",new_instance.count_words());

    //Extra method!!
    //This 'word_analysis' method shows a result of how many times repeated for each word from the given text 
    println!("\n\n\nWord frequency analysis for each word {:#?}",new_instance.word_analysis());
}



