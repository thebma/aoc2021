static DATA: &str = include_str!("../../data/day7.i");

fn sum_of_all_previous_numbers(num: u32) -> u32 {
    let mut sum = 0;
    for i in 0..num {
        sum += i
    }

    return num + sum;
}

fn get_fuel_expenditure(desired_position: i32, crab_positions: &Vec<i32>) -> u32
{
    let mut fuel_sum: u32 = 0;

    for crab_position in crab_positions 
    {
        let steps = (*crab_position - desired_position).abs() as u32;
        fuel_sum += sum_of_all_previous_numbers(steps);
    }

    return fuel_sum;
}

fn main() 
{
    let list_of_crabs: Vec<i32> = DATA
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let range = list_of_crabs.iter().max().unwrap();
    let mut position = 0;
    let mut fuel_counter = u32::MAX;

    for desired in 0..*range 
    {
        let total = get_fuel_expenditure(desired, &list_of_crabs);

        if total <= fuel_counter {
            position = desired;
            fuel_counter = total;
        }

        println!("{}", desired);
    }

    println!("Most fuel efficient arrangement is {} with position {}", fuel_counter, position);
}
