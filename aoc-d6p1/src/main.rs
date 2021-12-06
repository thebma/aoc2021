static DATA: &str = include_str!("../../data/day6.t");

fn main() 
{  
    let mut school = DATA
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let days = 256u32;

    for day in 0..days
    {
        let len = school.len();

        let dec_school = school
            .clone()
            .into_iter()
            .map(|x| x - 1)
            .collect::<Vec<i32>>();

        let mut new_school: Vec<i32> = Vec::new();

        for value in dec_school 
        {
            if value == -1 {
                new_school.push(6);
                new_school.push(8);
            }
            else {
                new_school.push(value)
            }
        }

        school = new_school;
        println!("Day {} \t {}", day, school.len());
    }

    println!("School size is {}", school.len());
}
