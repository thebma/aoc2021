use std::fs;

fn binary_to_decimal(binary: String) -> i32
{
    return i32::from_str_radix(&binary, 2).unwrap();
}

fn get_gamma(columns: u32, values: &Vec<&str>) -> i32
{
    let binary = get_common_bit("0", "1", columns, values);
    return binary_to_decimal(binary);
}  

fn get_epsilon(columns: u32, values: &Vec<&str>) -> i32
{
    let binary = get_common_bit("1", "0", columns, values);
    return binary_to_decimal(binary);
} 

fn get_common_bit(value0: &str, value1: &str, columns: u32, values: &Vec<&str>) -> String
{
    let mut gamma: String = String::new();

    for col in 0..columns 
    {
        let mut bit0 = 0i32;
        let mut bit1 = 0i32;

        for row in values 
        {
            let bit = row
                .chars()
                .nth(col as usize)
                .unwrap();

            if bit == '1' { bit1 += 1; } else { bit0 += 1};
        }

        gamma += if bit0 > bit1 { value0 } else { value1 };
    }

    return gamma;
}

fn load_diagnostic_data(columns: u32) -> (i32, i32)
{
    let content: String = fs::read_to_string("../data/day3.i")
        .expect("no diagnostic data found");


    let values = content
            .split_whitespace()
            .collect::<Vec<&str>>();

    (
        get_gamma(columns, &values),
        get_epsilon(columns, &values),
    )
}

fn main() {
    let (gamma, epsilon) = load_diagnostic_data(12);
    println!("Power consumption is {} omegawatts/h", gamma * epsilon);
}
