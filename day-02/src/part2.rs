use regex::Regex;

fn process_game(game: &str) -> i32 {
    let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
    let red_re = Regex::new(r"(?<red>\d+) red").unwrap();

    let splitted = game.split(';');

    let mut blue_min = 0;
    let mut green_min = 0;
    let mut red_min = 0;

    splitted.for_each(|case| {
        let blue = if let Some(blue_cap) = blue_re.captures(case) {
            blue_cap["blue"].parse().unwrap_or_default()
        } else { 0 };

        let green = if let Some(green_cap) = green_re.captures(case) {
            green_cap["green"].parse().unwrap_or_default()
        } else { 0 };

        let red = if let Some(red_cap) = red_re.captures(case) {
            red_cap["red"].parse().unwrap_or_default()
        } else { 0 };

        if blue > blue_min {
            blue_min = blue;
        }

        if green > green_min {
            green_min = green;
        }

        if red > red_min {
            red_min = red;
        }
    });

    blue_min * green_min * red_min
}

pub fn process(input: &str) -> i32 {
    let lines = input.lines();

    let result = lines.map(process_game).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    fn test_game(
        #[case] game: &str,
        #[case] power: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = process_game(game);
        assert_eq!(power, result);
        Ok(())
    }

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(2286, process(input));
        Ok(())
    }
}