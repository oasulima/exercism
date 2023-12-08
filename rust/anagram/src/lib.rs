use std::{cmp::Ordering, collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());

    let word_chars = word_to_chars(word);
    let word_lowercase = word.to_lowercase();

    for anagram in possible_anagrams
        .iter()
        .filter(|x| word_lowercase.cmp(&x.to_lowercase()) != Ordering::Equal)
    {
        let anagram_chars = word_to_chars(anagram);

        if word_chars == anagram_chars {
            result.insert(anagram);
        }
    }

    result
}

fn word_to_chars(word: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort();
    word_chars
}
