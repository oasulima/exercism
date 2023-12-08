pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if !list.is_empty() {
        for index in 0..list.len() - 1 {
            let first_word = list[index];
            let second_word = list[index + 1];
            result.push_str(&format!(
                "For want of a {first_word} the {second_word} was lost.\n"
            ));
        }
        let first_word = list[0];
        result.push_str(&format!("And all for the want of a {first_word}."));
    }
    result
}
