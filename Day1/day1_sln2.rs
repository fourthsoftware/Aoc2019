use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let mut result: u32 = 0;

    let f = File::open("day1_sln1_input.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let mut temp_fuel: u32 = line.unwrap().parse().unwrap();  

        let mut temp_result: u32 = 0;
        while temp_fuel >= 6 { // we don't want to go lower than 6 or we will get a negative answer
            temp_fuel = (temp_fuel / 3) - 2;
            temp_result += temp_fuel;
        }

        result += temp_result;
    }

    println!("{}", result);

}