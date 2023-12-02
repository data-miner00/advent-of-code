use regex::Regex;

fn process_game(game: &str) -> (i32, i32, i32, i32) {
    let id_re = Regex::new(r"\d+").unwrap();
    let blue_re = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let green_re = Regex::new(r"(?<green>\d+) green").unwrap();
    let red_re = Regex::new(r"(?<red>\d+) red").unwrap();

    let splitted = game.split(';');

    let id_cap = id_re.captures(game).unwrap();
    let id: i32 = id_cap.get(0).unwrap().as_str().parse().unwrap_or_default();
    let mut blue_max = 0;
    let mut green_max = 0;
    let mut red_max = 0;

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

        if blue > blue_max {
            blue_max = blue;
        }

        if green > green_max {
            green_max = green;
        }

        if red > red_max {
            red_max = red;
        }
    });

    (id, blue_max, green_max, red_max)
}

pub fn process(input: &str) -> i32 {
    let lines = input.lines();

    let output = lines.map(process_game);

    let red = 12;
    let green = 13;
    let blue = 14;

    let possible = output.filter(|(_, b, g, r)| *r <= red && *b <= blue && *g <= green);
    possible.fold(0, |acc, (id, _, _, _)| acc + id)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1, 6, 2, 4)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2, 4, 3, 1)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 3, 6, 13, 20)]
    fn test_game(
        #[case] game: &str,
        #[case] id: i32,
        #[case] blue: i32,
        #[case] green: i32,
        #[case] red: i32
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = process_game(game);

        assert_eq!(id, result.0);
        assert_eq!(blue, result.1);
        assert_eq!(green, result.2);
        assert_eq!(red, result.3);

        Ok(())
    }

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_str!("demo.txt");
        assert_eq!(8, process(input));
        Ok(())
    }
}