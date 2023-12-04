use std::collections::HashMap;

fn contains_exact_word(input_str: &str, target_word: &str) -> bool {
    let words: Vec<&str> = input_str.split_whitespace().collect();
    words.contains(&target_word)
}

pub fn process(input: &str) -> usize {
    let lines = input.lines();
    let mut hm: HashMap<usize, usize> = HashMap::new();
    let mut total = 0;

    for (index, x) in lines.enumerate() {
        let cleaned = x.replace("\"", "");
        let splitted = cleaned.split(":");
        let sanitized = splitted.filter(|&x| x.contains("|")).collect::<String>();
        let mut splitted = sanitized.split("|");
        let win = splitted.next().unwrap();
        let my = splitted.next().unwrap();

        let count: usize = my.split(" ").filter(|&x| x != "").map(|x| {
            if contains_exact_word(win, x) { 1 } else { 0 }
        }).sum();

        hm.insert(index, hm.get(&index).unwrap_or_else(|| &0) + 1);

        for card in 0..*hm.get(&index).unwrap() {
            for jndex in 1..=count {
                hm.insert(jndex + index, hm.get(&(jndex + index)).unwrap_or_else(|| &0) + 1);
            }
        }

        total += hm.get(&index).unwrap();
    }

    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(30, process(input));
        Ok(())
    }
}