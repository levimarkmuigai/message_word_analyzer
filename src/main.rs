use std::io;

use std::collections::HashSet;

fn main() {
   
    // Ask user for input
    println!("Enter your text for analysis: ");
    
    // assign input an empty container
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    
    // Trim the input
    let input = input.trim();

    message_analyzer(input);
}

// Function that analyses the message(sentence).
fn message_analyzer(message: &str) {
    
    // Let's the user know the program is working.
    println!("===========================================================================================================================");
    println!("Analysing your message: {}", message);

    // Splits the sentence into words.
    let words = message.split_whitespace();

    let pattern_analyzer_words: Vec<&str> = message.split_whitespace().collect(); 

     let (
            long_words_count, 
            repeats_count, 
            long_words, 
            repeated_words
         ) = pattern_analyzer(&pattern_analyzer_words);

    // Track the word count
    let mut word_count = 0;

    // Loop through the words to confirm what's there.
    for word in words {
        // Displays the words in the sentence and their length
        println!("Word: {}, Length: {}", word, word.len());

        // Increases the word count each iteration
        word_count += 1;
    }

    // Prints the word count 
    println!("Word count is: {}", word_count);

    // Prints the number of long words
    println!("Long words are: {}", long_words_count);

    // Prints the repeated words
    println!("Repeated words are: {}", repeats_count);

    // Prints the list of long words
    println!("Long words:");
    for word in &long_words {
        println!("- {}", word);
    }

    // Prints the list of repeated words
    println!("Repeated words:");
    for word in &repeated_words {
        println!("- {}", word);
    }
}

// Helper function that checks the length of words and repeated words.
fn pattern_analyzer(words: &[&str]) -> (usize, usize, Vec<String>, HashSet<String>){
    
    // Count the number of character in a word and checks if greater than twelve.
    let mut twelve_or_more_count = 0;

    // Creates an empty Vec to store long words.
    let mut twelve_or_more_words_vec = Vec::new();

    // Creates a set for comparison to current word.
    let mut seen_words = HashSet::new();

    // Starts of the count for repeated words
    let mut repeated_words_count = 0;

    // Empty set to store repeated words.
    let mut repeated_words_set = HashSet::new();

    // Loops through each word in words
    for &word in words {
        
        /*
         * Checks if length of a word if equal 
         * or greater than 12 characters or more
        */
        if word.len() >= 12 {
            twelve_or_more_count += 1; // Updates count if the word 
                                       // is 12 charcters or more.
                            
            twelve_or_more_words_vec.push(word.to_string()); // Clones the long word to the Vec and
                                                            // converts it from  a String slice to string.
        };


        // Changes words to lowercase
        let word_lower = word.to_lowercase();

        // 
        if !seen_words.insert(word_lower.clone()) {
            repeated_words_count += 1; // Updates count if words are the same

            repeated_words_set.insert(word_lower);
            
        }
    }

    

    // Returns a tuple of counts, long words and repeated words.
    (
        twelve_or_more_count, 
        repeated_words_count, 
        twelve_or_more_words_vec, 
        repeated_words_set
    ) 
}
