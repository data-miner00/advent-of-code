pub fn process(input: &str) -> i32 {
    todo!("very difficult");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(281, process(input));
        Ok(())
    }
}