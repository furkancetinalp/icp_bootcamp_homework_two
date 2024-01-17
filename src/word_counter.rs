use std::collections::HashMap;

//defining WordCounter struct
//just because this file is created as independent file from main.rs, we need explicitly type public including parameters
//in this way, main.rs can reach that struct
pub struct WordCounter{
    pub text :String, 
}

//trait is used instead of struct implementation
//reason is that each method is a kind of behaviour for struct 'WordCounter'
//just like an interface definition of a class
//this allows the program to be easily expanded and scaled
pub trait WordActions {
    fn new(text:&str) -> WordCounter;
    fn count_words(&self) -> usize;
    fn word_analysis(&self) -> HashMap<&str,usize>;
}

//implementation of the traif for the 'WordCounter'
impl WordActions for WordCounter {
    //method that creates an instance of WordCounter
    fn new(text:&str) -> WordCounter {
        WordCounter{text:text.to_string()}
    }

    //method that counts the words
    fn count_words(&self) -> usize {
        let data :Vec<&str> = self.text.trim().split_whitespace().collect();
        data.len()
    }

    //method that shows how many times each word repeated 
    //format: HashMap => {'word','number_of_times_repeated'}
    fn word_analysis(&self) -> HashMap<&str,usize> {
        let mut frequency: HashMap<&str, usize> = HashMap::new();
        let data :Vec<&str> = self.text.trim().split_whitespace().collect();

        for word in data {
            *frequency.entry(word).or_insert(0) += 1;
        }
        return frequency;
    }
}