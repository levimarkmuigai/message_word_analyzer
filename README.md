# Message analyzer
A rust CLI tool that analyzes the words in a message input, it then gives out various analysis like:

- Word length for each word
- Word count
- Repeated words
- Long words

## 🗂️ Project Structure
```sh
message_word_analyzer/
├── src/
│   └── main.rs      // Main program logic
├── Cargo.toml       // Rust project config
└── README.md        // This file
```

## 🏃🏾‍♂️‍➡️ How to run
1. Clone the repo
```sh
git clone https://github.com/levimarkmuigai/message_word_analyzer.git
```
2. Change into project source
```sh
cd message_word_analyzer/src
```
3. Run the program
```sh
cargo run
```

## 🧏🏾‍♀️ Example input and out put
```sh
  cargo run
   Compiling message_word_analyzer v0.1.0 (/home/levi/Projects/Rust/message_word_analyzer)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `/home/levi/Projects/Rust/message_word_analyzer/target/debug/message_word_analyzer`
Enter your text for analysis: 
Communication is essential in international collaboration, especially when Communication breaks down.
===========================================================================================================================
Analysing your message: Communication is essential in international collaboration, especially when Communication breaks down.
Word: Communication, Length: 13
Word: is, Length: 2
Word: essential, Length: 9
Word: in, Length: 2
Word: international, Length: 13
Word: collaboration,, Length: 14
Word: especially, Length: 10
Word: when, Length: 4
Word: Communication, Length: 13
Word: breaks, Length: 6
Word: down., Length: 5
Word count is: 11
Long words are: 4
Repeated words are: 1
Long words:
- Communication
- international
- collaboration,
- Communication
Repeated words:
- communication

```

