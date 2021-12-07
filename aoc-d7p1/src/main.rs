static DATA: &str = include_str!("../../data/day7.i");

fn get_fuel_expenditure(desired_position: i32, crab_positions: &Vec<i32>) -> u32
{
    let mut fuel_sum: u32 = 0;

    for crab_position in crab_positions 
    {
        fuel_sum += (*crab_position - desired_position).abs() as u32;
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
    println!("Searching from range of 0 to {}..", range);

    let mut position = 0;
    let mut fuel_counter = u32::MAX;

    for desired in 0..*range 
    {
        let total = get_fuel_expenditure(desired, &list_of_crabs);

        if total <= fuel_counter {
            position = desired;
            fuel_counter = total;
        }
    }

    println!("Most fuel efficient arrangement is {} with position {}", fuel_counter, position);

}
