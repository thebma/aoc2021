use std::cmp;

static DATA: &str = include_str!("../../data/day5.i");

fn path_of_segment(start: &Vec<i32>, end: &Vec<i32>) -> Vec<Vec<i32>>
{
    let mut lines: Vec<Vec<i32>> = Vec::new();
    let dx = (start[0] - end[0]).abs();
    let dy = (start[1] - end[1]).abs();

    //Is this a row or a column?
    if dx == 0 && dy == 0 
    {
        //Columns...
        if dx == 0 
        {
            let basis = cmp::min(start[1], end[1]); //Which of the Y values is lower...

            for i in 0..dy+1
            {
                lines.push(vec![start[0], basis + i])
            }
        }
        //Rows..
        else if dy == 0 
        {
            let basis = cmp::min(start[0], end[0]); //Which of the X values is lower...

            for i in 0..dx+1 
            {
                lines.push(vec![basis + i, end[1]]);
            }
        }
    }

    return lines;
}


fn main() 
{
    let segments = DATA
        .split_whitespace()
        .filter(|a| a != &"->")        
        .flat_map(|s|{
            s.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<i32>>() 
        .chunks(2)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<i32>>>()
        .chunks(2)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<Vec<i32>>>>();

    let board_x = segments.clone()
        .into_iter()
        .into_iter()
        .flatten()
        .map(|x| x[0])
        .max()
        .unwrap();

    let board_y = segments.clone()
        .into_iter()
        .into_iter()
        .flatten()
        .map(|x| x[0])
        .max()
        .unwrap();

    //Create a 2D Vector of size max(x,y) by max(x,y)...
    let cap =  cmp::max(board_x as usize, board_y as usize);
    let mut occupied: Vec<Vec<u32>> = Vec::with_capacity(cap);

    for x in 0..board_x+1
    {
        occupied.push(Vec::with_capacity(cap));

        for _ in 0..board_y+1
        {
            occupied[x as usize].push(0);
        }
    }

    //Put all our paths onto it.
    for segment in &segments 
    {
        let paths =  path_of_segment(&segment[0], &segment[1]);

        for path in paths 
        {
            occupied[path[1] as usize][path[0] as usize] += 1;
        }
    }

    //Evaluate which lines that are higher than 1 (i.e. they intersected.)
    let mut count = 0;
    for row in 0..board_x+1 {
        for col in 0..board_y+1 {
            count += if occupied[row as usize][col as usize] >= 2 { 1 } else { 0 } 
        }
    }

    println!("{} intersections", count);
}