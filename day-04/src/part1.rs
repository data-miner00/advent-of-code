fn points(nth: i32) -> i32 {
    match nth {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => points(nth - 1) * 2
    }
}

fn contains_exact_word(input_str: &str, target_word: &str) -> bool {
    let words: Vec<&str> = input_str.split_whitespace().collect();
    words.contains(&target_word)
}

pub fn process(input: &str) -> i32 {
    let lines = input.lines();

    lines.map(|x| {
        let cleaned = x.replace("\"", "");
        let splitted = cleaned.split(":");
        let sanitized = splitted.filter(|&x| x.contains("|")).collect::<String>();
        let mut splitted = sanitized.split("|");
        let win = splitted.next().unwrap();
        let my = splitted.next().unwrap();

        let count = my.split(" ").filter(|&x| x != "").map(|x| {
            if contains_exact_word(win, x) { 1 } else { 0 }
        }).sum();

        points(count)
    }).fold(0, |a, b| a + b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(13, process(input));
        Ok(())
    }
}