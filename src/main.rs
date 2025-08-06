use std::io;

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
    println!("Analysing your message {}", message);

    // Splits the sentence into words.
    let words = message.split_whitespace();

    // Track the word count
    let mut word_count = 0;

    // Loop through the words to confirm what's there.
    for word in words {
        // Displays the words in the sentence and their length
        println!("Word: {}, Length: {}", word, word.len());
        
        // Increases the word count each iteration
        word_count += 1;
    }

    println!("Word count is: {}", word_count);
}
