fn most_frequent_word(text: &str) -> (String, usize) {
    // Collect words into a Vec using string slices (borrowing from text)
    let words: Vec<&str> = text.split_whitespace().collect();

    // Parallel arrays to track unique words and their counts
    let mut unique_words: Vec<&str> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();

    // Count frequencies using mutable references
    for word in &words {
        let mut found = false;

        // Search existing words
        for i in 0..unique_words.len() {
            if unique_words[i] == *word {
                // Use mutable reference to increment count in-place
                let count_ref: &mut usize = &mut counts[i];
                *count_ref += 1;
                found = true;
                break;
            }
        }

        // New word encountered
        if !found {
            unique_words.push(word);
            counts.push(1);
        }
    }

    // Find the word with the highest frequency
    let mut max_index = 0;
    for i in 1..counts.len() {
        if counts[i] > counts[max_index] {
            max_index = i;
        }
    }

    (unique_words[max_index].to_string(), counts[max_index])
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
