use std::{
    collections::HashMap,
    thread::{self},
};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        n if n < 700 => count_chars(input),
        _ => count_in_parallel(input, worker_count),
    }
}

fn count_in_parallel(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();

    thread::scope(|s| {
        let mut handlers = Vec::with_capacity(worker_count);
        for chunk in input.chunks(input.len() / worker_count + 1) {
            let handler = s.spawn(|| count_chars(chunk));
            handlers.push(handler);
        }

        for handler in handlers {
            let data = handler.join().unwrap();
            merge_counters(&mut result, &data);
        }
    });

    result
}

fn count_chars(input: &[&str]) -> HashMap<char, usize> {
    let mut letters: HashMap<char, usize> = HashMap::new();
    for str in input.iter() {
        for char in str
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            *letters.entry(char).or_default() += 1;
        }
    }
    letters
}

fn merge_counters(result: &mut HashMap<char, usize>, data: &HashMap<char, usize>) {
    for (new_char, new_counter) in data {
        *result.entry(*new_char).or_default() += new_counter;
    }
}
