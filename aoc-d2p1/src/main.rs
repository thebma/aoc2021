use std::fs;

fn read_course() -> Vec<String>
{   
    let file_content = fs::read_to_string("../data/day2.i")
        .expect("could not load data");

    let result = file_content
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    result
}

fn main() {
    let course_data: Vec<String> = read_course();

    let mut depth = 0i32;
    let mut horizontal = 0i32;

    for value in course_data 
    {
        let ws: Vec<&str> = value.split_whitespace().collect();
        let key = ws[0].chars().next().unwrap();
        let value = ws[1].parse::<i32>().unwrap();

        match key {
            'f' => { horizontal += value },
            'd' => { depth += value },
            'u' => { depth -= value },
            _ => { panic!("unknown command") }
        }
    }

    println!("answer is {}", horizontal * depth);
}
