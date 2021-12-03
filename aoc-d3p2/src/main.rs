use std::fs;

fn binary_to_decimal(binary: String) -> i32
{
    return i32::from_str_radix(&binary, 2).unwrap();
}

fn get_oxygen(columns: u32, values: &Vec<&str>) -> i32
{
    return get_value_from_bit_criterion(
        "0", "1", "1", columns, values
    )
}

fn get_co2(columns: u32, values: &Vec<&str>) -> i32
{
    return get_value_from_bit_criterion(
        "1", "0", "0", columns, values
    )
}

fn get_value_from_bit_criterion(
    bit0: &str, bit1: &str, eq: &str,
    columns: u32, values: &Vec<&str>
) -> i32
{
    let mut bits: String = String::new();

    for i in 0..columns 
    {
        let (b0, b1, last, last_val) = 
            get_common_bit_for_column(i, values, &bits);

        if last 
        {
            bits = last_val;
            break;
        }
        
        if b0 == b1 { bits += eq } 
        else if b0 > b1 { bits += bit0 } 
        else { bits += bit1 };
    }

    return binary_to_decimal(bits);
}

fn get_bit_of_column(row: &str, column: u32) -> char
{
    return row.chars().nth(column as usize).unwrap();
}

fn get_common_bit_for_column(
    column: u32, values: &Vec<&str>, bits: &str
) -> (i32, i32, bool, String)
{
    let mut bit0 = 0i32;
    let mut bit1 = 0i32;

    let mut last_binary: &str = "";
    let mut processed: u32 = 0;

    for row in values 
    {
        if row.starts_with(bits)
        {
            
            let bit = get_bit_of_column(row, column);
            if bit == '1' { bit1 += 1; } else { bit0 += 1; };
            
            processed += 1;
            last_binary = row;
        }
    }

    //If we processed all the way to the last element.
    if processed <= 1 {
        return (0, 0, true, String::from(last_binary));
    }

    return (bit0, bit1, false, String::new());
}

fn load_diagnostic_data(columns: u32) -> (i32, i32)
{
    let content: String = fs::read_to_string("../data/day3.i")
        .expect("no diagnostic data found");

    let values = content
            .split_whitespace()
            .collect::<Vec<&str>>();

    (
        get_oxygen(columns, &values),
        get_co2(columns, &values)
    )
}

fn main() {
    let (oxygen, co2) = load_diagnostic_data(12);
    println!("Habitability = {} * {} = {} points", 
    oxygen, co2, oxygen * co2);
}
