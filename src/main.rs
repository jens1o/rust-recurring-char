use std::collections::HashSet;
use std::env;
use std::process;

fn main() {
    let word = env::args().nth(1);

    let word = word.unwrap_or_else(|| {
        eprintln!("You need to enter a word to check whether it contains a recurring char.");
        eprintln!("Usage: cargo run <word>");

        process::exit(1);
    });

    let recurring_char = get_first_recurring_char(&word);

    match recurring_char {
        Some(ref recurring_char) => {
            println!(
                r#"The word "{}" has an recurring character: {}"#,
                word, recurring_char
            );
        }
        None => {
            println!("The word {} doesn't have an recurring character.", word);
        }
    }
}

fn get_first_recurring_char(slice: &str) -> Option<char> {
    // do a good guess to avoid allocating over and over again.
    let mut seen_chars: HashSet<char> = HashSet::with_capacity(slice.len() / 2);

    for character in slice.chars() {
        if seen_chars.contains(&character) {
            return Some(character);
        }

        seen_chars.insert(character);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::get_first_recurring_char;

    #[test]
    fn test_none() {
        assert_eq!(get_first_recurring_char("ABCDEF"), None);
    }

    #[test]
    fn simple_recurring() {
        assert_eq!(get_first_recurring_char("DBCABA"), Some('B'));
    }
}
