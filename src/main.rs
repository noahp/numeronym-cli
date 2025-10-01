use clap::Parser;

/// A CLI tool that generates numeronym abbreviations from text
///
/// A numeronym is a number-based word abbreviation where the number represents
/// the count of letters between the first and last letter of a word.
/// For example: "internationalization" becomes "i18n" (18 letters between 'i' and 'n')
#[derive(Parser)]
#[command(name = "numeronym")]
#[command(about = "Generate numeronym abbreviations from text")]
#[command(version = "0.1.0")]
struct Args {
    /// Text to convert to numeronyms
    #[arg(required = true)]
    text: Vec<String>,
}

/// Generate a numeronym from a word, preserving punctuation
fn generate_numeronym(word: &str) -> String {
    // Separate alphabetic characters from punctuation
    let mut result = String::new();
    let mut current_word = String::new();

    for ch in word.chars() {
        if ch.is_alphabetic() {
            current_word.push(ch);
        } else {
            // Process accumulated word if any
            if !current_word.is_empty() {
                result.push_str(&create_numeronym(&current_word));
                current_word.clear();
            }
            // Add punctuation as-is
            result.push(ch);
        }
    }

    // Process any remaining word
    if !current_word.is_empty() {
        result.push_str(&create_numeronym(&current_word));
    }

    result
}

/// Create numeronym from a pure alphabetic word
fn create_numeronym(word: &str) -> String {
    let len = word.len();

    match len {
        0 => String::new(),
        1 | 2 => word.to_string(),
        _ => {
            let chars: Vec<char> = word.chars().collect();
            let first = chars[0];
            let last = chars[len - 1];
            let middle_count = len - 2;
            format!("{}{}{}", first, middle_count, last)
        }
    }
}

fn main() {
    let args = Args::parse();

    // Join all arguments into a single text string
    let input_text = args.text.join(" ");

    // Split by whitespace and process each word
    let result: Vec<String> = input_text
        .split_whitespace()
        .map(|word| generate_numeronym(word))
        .collect();

    // Print the result
    println!("{}", result.join(" "));
}
