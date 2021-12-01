use std::fs;
use std::env;

fn load_sonar_data() -> Vec<i32>
{
    let mut out: Vec<i32> = Vec::new();

    let content = fs::read_to_string("./sonar.txt")
        .expect("could not find sonar");

    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let value: i32 = match line.parse() {
            Ok(num) => num,
            Err(_) => panic!("invalid sonar data.")
        };

        out.push(value);
    }

    out 
}

fn main() {
    let sonar_data = load_sonar_data();

    let mut last_sonar: i32 = 0;
    let mut increments: u32 = 0;

    for depth in sonar_data
    {
        if last_sonar > 0 
        {
            let delta = depth - last_sonar;

            if delta > 0
            {
                increments += 1;
            }
        }
        else
        {
            println!("First element.");
        }

        last_sonar = depth;
    }

    println!("We had {} increments", increments);
}