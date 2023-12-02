use day_02::part2::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("../input.txt");
    let result = process(file);
    println!("{}", result);
    Ok(())
}
