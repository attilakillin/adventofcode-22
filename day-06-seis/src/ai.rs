// These functions were written by OpenAI's new ChatGPT AI based on single-sentence prompts.

use std::collections::HashMap;

// Number of prompts: 2, one for the function itself, and one specifying
// that I need a return value too.
pub fn read_string_until_four_different_chars(s: &str) -> usize {
    let mut last_four_chars = Vec::new();
    let mut count = 0;

    for c in s.chars() {
        last_four_chars.push(c);
        count += 1;

        if last_four_chars.len() > 4 {
            last_four_chars.remove(0);
        }

        if last_four_chars.len() == 4 &&
            last_four_chars[0] != last_four_chars[1] &&
            last_four_chars[0] != last_four_chars[2] &&
            last_four_chars[0] != last_four_chars[3] &&
            last_four_chars[1] != last_four_chars[2] &&
            last_four_chars[1] != last_four_chars[3] &&
            last_four_chars[2] != last_four_chars[3]
        {
            break;
        }
    }

    count
}

// Number of prompts: 2, one for the solution itself, and one specifying that using
// a hashmap for comparisons is better than 14*14 simple comparison.
pub fn read_string_until_fourteen_different_chars(s: &str) -> usize {
    let mut last_fourteen_chars = Vec::new();
    let mut char_counts = HashMap::new();
    let mut count = 0;

    for c in s.chars() {
        last_fourteen_chars.push(c);
        count += 1;

        *char_counts.entry(c).or_insert(0) += 1;

        if last_fourteen_chars.len() > 14 {
            let c = last_fourteen_chars.remove(0);
            *char_counts.entry(c).or_insert(0) -= 1;
        }

        if last_fourteen_chars.len() == 14 && char_counts.values().all(|&v| v <= 1) {
            break;
        }
    }

    count
}
