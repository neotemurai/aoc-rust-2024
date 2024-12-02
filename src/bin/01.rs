use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Create two vectors to store the numbers
        let mut column1 = Vec::new();
        let mut column2 = Vec::new();

        // Read the file line by line
        for line in reader.lines() {
            let line = line?; // Use ? to handle potential errors
            // Split the line by whitespace
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                // Parse the numbers and add them to the vectors
                let num1: usize = parts[0].parse().unwrap_or(0);
                let num2: usize = parts[1].parse().unwrap_or(0);

                column1.push(num1);
                column2.push(num2);
            }
        }

        let mut sum:usize = 0;
        column1.sort();
        column2.sort();
        for i in 0..column1.len(){
            sum += (column1[i]).abs_diff(column2[i]);
        }

        Ok(sum) // Return Ok with the result
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
