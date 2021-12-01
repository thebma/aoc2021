use std::fs;

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

fn sliding_window(input: &Vec<i32>, idx: usize) -> (bool, i32)
{
    //We are at the end of the file where we cannot compose another sliding window.
    if (idx + 3) >= input.len() { return (false, 0); } 

    //Both sliding window A and B always have these values.
    let common = input[idx + 1] + input[idx + 2];

    //Calculate the difference between A and B (and completing the sliding window sequence).
    let delta = common + input[idx + 3] - (input[idx] + common);
    return (true, delta);
}

fn main() 
{
    let sonar_data = load_sonar_data();
    let mut increments: u32 = 0;

    for i in 0..sonar_data.len() 
    {
        let (found, delta) = sliding_window(&sonar_data, i);

        //Break when we cannot create another sliding window due to EOF.
        if !found { break; }

        //Increment if the delta is positive aka an increase.
        increments += if delta > 0 { 1 } else { 0 };
    }

    println!("We had {} increments", increments);
}