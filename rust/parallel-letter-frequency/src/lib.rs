use std::{
    collections::HashMap,
    thread::{self},
};

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    let mut handlers = vec![];

    for chunk in input.chunks(worker_count) {
        let chunk = chunk.to_owned();
        let handler = thread::spawn(move || {
            let mut letters: Box<HashMap<char, usize>> = Box::new(HashMap::new());
            for str in chunk.iter() {
                for char in str.chars() {
                    if !char.is_alphabetic() {
                        continue;
                    }

                    let char = char.to_ascii_lowercase();

                    let counter = letters.get_mut(&char);
                    if let Some(counter) = counter {
                        *counter += 1;
                    } else {
                        letters.insert(char, 1);
                    }
                }
            }
            letters
        });
        handlers.push(handler);
    }
    for handler in handlers {
        let data = handler.join().unwrap();
        merge_counters(&mut result, &data);
    }

    result
}

fn merge_counters(result: &mut HashMap<char, usize>, data: &HashMap<char, usize>) {
    for (new_char, new_counter) in data {
        let counter = result.get_mut(&new_char);
        if let Some(counter) = counter {
            *counter += new_counter;
        } else {
            result.insert(*new_char, *new_counter);
        }
    }
}
