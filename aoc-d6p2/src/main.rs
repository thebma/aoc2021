static DATA: &str = include_str!("../../data/day6.i");

fn fish_shift(fish_register: Vec<u64>) -> Vec<u64>
{
    let mut old_fish_register = fish_register.clone();
    let mut new_fish_register = vec![0; 10];

    if old_fish_register[0] > 0 
    {
        old_fish_register[7] += old_fish_register[0];
        old_fish_register[9] += old_fish_register[0];
    }
    
    //Shift all the values to the left. (minus the extra space);
    for i in 0..9
    {   
        let one_over = (i + 1) % 9;
        new_fish_register[i] = old_fish_register[one_over];
    }
    
    return new_fish_register;
}

fn main() 
{  
    let mut school = DATA
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fish_register: Vec<u64> = vec![0; 10];

    for fish in school {
        fish_register[fish as usize] += 1;
    }

    let mut running_fish_register: Vec<u64> = fish_register;
    println!("Initial State: {:?}", running_fish_register);

    for day in 0..256
    {
        running_fish_register = 
            fish_shift(running_fish_register)
            .clone();

        println!("After Day {}: {:?}", day + 1, running_fish_register);
    }    

    let mut sum = 0;
    for all in running_fish_register 
    {
        sum += all;
    }

    println!("{}", sum);


    // let days = 256u32;

    // for day in 0..days
    // {
    //     let len = school.len();

    //     let dec_school = school
    //         .clone()
    //         .into_iter()
    //         .map(|x| x - 1)
    //         .collect::<Vec<i32>>();

    //     let mut new_school: Vec<i32> = Vec::new();

    //     for value in dec_school 
    //     {
    //         if value == -1 {
    //             new_school.push(6);
    //             new_school.push(8);
    //         }
    //         else {
    //             new_school.push(value)
    //         }
    //     }

    //     school = new_school;
    //     println!("Day {} \t {}", day, school.len());
    // }

    //println!("School size is {}", school.len());
}
