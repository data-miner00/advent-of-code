pub fn process(input: &str) -> i32 {
    let lines = input.lines();

    lines.map(|x| {
        let iterator = x.chars().filter(|x| x.is_digit(10));
        let first = iterator.clone().next().unwrap();
        let last = iterator.rev().next().unwrap();

        let combined = format!("{}{}", first, last);

        match combined.parse::<i32>() {
            | Ok(integer) => integer,
            | _ => 0
        }
    }).fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo1.txt");
        assert_eq!(142, process(input));
        Ok(())
    }
}