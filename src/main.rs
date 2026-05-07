use clap::Parser;
use std::collections::BTreeMap;
use std::fs;

/// A CLI tool that generates numeronym abbreviations from text
///
/// A numeronym is a number-based word abbreviation where the number represents
/// the count of letters between the first and last letter of a word.
/// For example: "internationalization" becomes "i18n" (18 letters between 'i' and 'n')
#[derive(Parser)]
#[command(name = "numeronym")]
#[command(about = "Generate numeronym abbreviations from text")]
#[command(version = "0.2.0")]
struct Args {
    /// Text to convert to/from numeronyms
    #[arg(required = true)]
    text: Vec<String>,

    /// Reverse mode: expand numeronyms to possible words
    #[arg(long)]
    undo: bool,

    /// Dictionary file for --undo mode
    #[arg(long, default_value = "/usr/share/dict/words")]
    dict: String,
}

fn create_numeronym(word: &str) -> String {
    let len = word.len();
    match len {
        0 => String::new(),
        1 | 2 => word.to_string(),
        _ => {
            let chars: Vec<char> = word.chars().collect();
            format!("{}{}{}", chars[0], len - 2, chars[len - 1])
        }
    }
}

fn generate_numeronym(word: &str) -> String {
    let mut result = String::new();
    let mut current_word = String::new();

    for ch in word.chars() {
        if ch.is_alphabetic() {
            current_word.push(ch);
        } else {
            if !current_word.is_empty() {
                result.push_str(&create_numeronym(&current_word));
                current_word.clear();
            }
            result.push(ch);
        }
    }
    if !current_word.is_empty() {
        result.push_str(&create_numeronym(&current_word));
    }
    result
}

/// Parse a numeronym like "o11y" into (first_char, middle_count, last_char).
/// Returns None if the input doesn't look like a numeronym.
fn parse_numeronym(s: &str) -> Option<(char, usize, char)> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 3 {
        return None;
    }
    let first = chars[0];
    let last = *chars.last().unwrap();
    if !first.is_alphabetic() || !last.is_alphabetic() {
        return None;
    }
    let middle: String = chars[1..chars.len() - 1].iter().collect();
    let count: usize = middle.parse().ok()?;
    Some((first, count, last))
}

fn expand_numeronym(numeronym: &str, dict_words: &[String]) -> Vec<String> {
    match parse_numeronym(numeronym) {
        None => {
            // Not a valid numeronym pattern — return the input unchanged
            vec![numeronym.to_string()]
        }
        Some((first, middle_count, last)) => {
            let target_len = middle_count + 2;
            let first_lower = first.to_lowercase().next().unwrap();
            let last_lower = last.to_lowercase().next().unwrap();

            let mut matches: Vec<String> = dict_words
                .iter()
                .filter(|w| {
                    let wl: Vec<char> = w.chars().collect();
                    wl.len() == target_len
                        && wl[0].to_lowercase().next().unwrap() == first_lower
                        && wl[wl.len() - 1].to_lowercase().next().unwrap() == last_lower
                        && wl.iter().all(|c| c.is_alphabetic())
                })
                .cloned()
                .collect();

            matches.sort_by_key(|w| w.to_lowercase());
            matches.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
            matches
        }
    }
}

fn load_dict(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn main() {
    let args = Args::parse();
    let input_text = args.text.join(" ");

    if args.undo {
        let dict = load_dict(&args.dict);
        // Group expansions by numeronym token for clean output
        let tokens: Vec<&str> = input_text.split_whitespace().collect();
        let mut results: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for token in &tokens {
            let key = token.to_string();
            if !results.contains_key(&key) {
                results.insert(key, expand_numeronym(token, &dict));
            }
        }
        for token in &tokens {
            let matches = &results[*token];
            if matches.len() == 1 && matches[0] == *token {
                // Not a numeronym, print as-is
                println!("{}: (not a numeronym)", token);
            } else if matches.is_empty() {
                println!("{}: (no matches found)", token);
            } else {
                println!("{}:", token);
                for m in matches {
                    println!("  {}", m);
                }
            }
        }
    } else {
        let result: Vec<String> = input_text
            .split_whitespace()
            .map(|word| generate_numeronym(word))
            .collect();
        println!("{}", result.join(" "));
    }
}
