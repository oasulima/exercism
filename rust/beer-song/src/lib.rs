pub fn verse(n: u32) -> String {
    match n {
        0 => verse_0(),
        1 => verse_1(),
        _ => verse_more_1(n),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}

fn verse_0() -> String {
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n" .to_string()
}

fn verse_1() -> String {
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
}

fn verse_more_1(num: u32) -> String {
    let new_num = num - 1;
    let ending = if new_num > 1 { "s" } else { "" };
    format!("{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {new_num} bottle{ending} of beer on the wall.\n")
}
