use std::fs;

fn aggregate(acc: Result<(i32, i32), String>, calorie_count: &str) -> Result<(i32, i32), String> {
  match (acc, calorie_count) {
    (Ok((max, current)), "") if current > max => Ok((current, 0)),
    (Ok((max, current)), "") if max > current => Ok((max, 0)),
    (Ok((max, current)), _)                   => calorie_count.parse::<i32>()
                                                   .map(|i| (max, i + current)) 
                                                   .map_err(|_| "Unable to parse.".to_string()),
    (err, _)                                  => err
  }
}

fn parse_input(all_calorie_counts: String) -> Result<i32, String> {
  all_calorie_counts
    .lines()
    .fold(Ok((0,0)), aggregate)
    .map(|results| results.0)
}

fn main() {
  let result = fs::read_to_string("input.txt")
    .map_err(|_| "Ran into an error trying to read from input file: input.txt".to_string())
    .and_then(parse_input);

  match result {
    Ok(elf_with_max) => println!("Elf with the highest count: {elf_with_max}"),
    Err(err)         => println!("{err}")
  }
}
