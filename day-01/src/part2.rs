fn mapping(input: &str) -> i32 {
    match input {
        | "one" | "eno" | "1" => 1,
        | "two" | "owt" | "2" => 2,
        | "three" | "eerht" | "3" => 3,
        | "four" | "ruof" | "4" => 4,
        | "five" | "evif" | "5" => 5,
        | "six" | "xis" | "6" => 6,
        | "seven" | "neves" | "7" => 7,
        | "eight" | "thgie" | "8" => 8,
        | "nine" | "enin" | "9" => 9,
        | _ => 0,
    }
}

pub fn process(input: &str) -> i32 {
    let lines = input.lines();
    let search_list = [
        "one", "two", "three",
        "four", "five", "six",
        "seven", "eight", "nine",
        "1", "2", "3", "4", "5",
        "6", "7", "8", "9"
    ];

    let rev_search_list: Vec<String> = search_list.iter().map(|s| s.chars().rev().collect()).collect();

    lines.map(|line| {

        let reversed: String = line.chars().rev().collect();

        let searched = search_list.map(|criteria| {
            let index = match line.find(criteria) {
                | Some(index) => index as i32,
                | None => -1
            };

            (criteria, index)
        });

        let searched_reversed = rev_search_list.iter().map(|criteria| {
            let index = match reversed.find(criteria) {
                | Some(index) => index as i32,
                | None => -1
            };

            (criteria, index)
        });
        
        let searched = searched.iter().filter(|&&(_, index)| index != -1);
        let (first, _) = searched.min_by_key(|(_, index)| index).unwrap();
        let searched_reversed = searched_reversed.filter(|&(_, index)| index != -1);
        let (last, _) = searched_reversed.min_by_key(|&(_, index)| index).unwrap();

        mapping(first) * 10 + mapping(last)
    }).fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo2.txt");
        assert_eq!(281, process(input));
        Ok(())
    }
}