use std::fs;

#[derive(Debug, Clone)]
pub struct Board
{
    rows: u32,
    columns: u32,
    completing_number: u32,
    step_to_complete: u32,
    completed: bool,
    board: Vec<u32>,
    marked: Vec<u32>
}

impl Board 
{
    pub fn evaluate(mut self, num: u32) -> Board
    {
        if self.completed { return self }

        self.marked.push(num);
        self.completing_number = num;
        self.step_to_complete += 1; //this is basically self.marked.len()... whatever...
        return self;
    }

    pub fn is_complete(mut self) -> bool
    {
        if self.completed { return true }

        let mut combinations_to_check: Vec<Vec<u32>> = Vec::new();

        //All the rows...
        for row in self.board
            .clone()
            .chunks(5)
            .collect::<Vec<&[u32]>>()
        {
            combinations_to_check.push(row.to_vec());
        }

        //All columns:
        let chunks =  self.board
            .chunks(5)
            .collect::<Vec<&[u32]>>();

        for cidx in 0..self.columns
        {
            let mut cvec: Vec<u32> = Vec::new();

            for ridx in 0..self.rows 
            {
                cvec.push(chunks[ridx as usize][cidx as usize]);
            }

            combinations_to_check.push(cvec);
        }

        for combination in combinations_to_check 
        {
            let mut match1 = 0;

            for combo_value in combination.clone()
            {
                if self.marked.contains(&combo_value)
                {
                    match1 += 1;
                }
            }
            
            if match1 == self.columns 
            {
                self.completed = true;
                return true;
            }
        }
        

        self.completed = false;
        return false;
    }

    pub fn sum_unmarked(self) -> u32
    {
        let mut sum = 0u32;

        for board_value in self.board 
        {
            if !self.marked.contains(&board_value)
            {
                sum += board_value;
            }
        }

        return sum;
    }
}

fn load_draws() -> Vec<u32>
{   
    let data: String = load_data();

    let draws = data
        .split_whitespace()
        .take(1)
        .collect::<Vec<&str>>();

    let result = draws
        .get(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    
    return result;
}

fn load_boards() -> Vec<Board>
{   
    let mut boards: Vec<Board> = Vec::new();

    let data: String = load_data();
    let board_count = (data.split("\n").collect::<Vec<&str>>().len() - 1) / 6;

    for board_index in 0..board_count + 1 
    {     
        let board_data = data
            .split("\n\r")
            .skip(board_index)
            .next().unwrap();

        let board_rows = board_data
            .split("\n")
            .skip(1)
            .filter(|s|!s.trim().is_empty())
            .collect::<Vec<&str>>();

        let board_row_size: u32 = board_rows.len().clone() as u32;
        if board_row_size == 0 { continue; }

        let mut parsed_board: Board = Board {
            rows: board_row_size,
            columns: 0,
            completing_number: 0,
            step_to_complete: 0,
            completed: false,
            board: Vec::new(),
            marked: Vec::new()
        };

        for row in board_rows 
        {
            let vals = row
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| str::replace(s, "\r", ""))
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let vals_size: u32 = vals.len().clone() as u32;
            parsed_board.columns = vals_size; //This could mess up logic if not a uniform N by N board.

            for value in vals 
            {
                parsed_board.board.push(value);
            }
        }

        boards.push(parsed_board);
    }

    return boards;

}

fn load_data() -> String
{
    let data = fs::read_to_string("../data/day4.i")
        .expect("could not load day 4.");

    return data;
}


fn main()
{
    let draws: Vec<u32> = load_draws();

    for board in load_boards() 
    {
        let mut temp = board.clone();

        for draw in &draws
        {
            //This is so messed up...
            temp = temp.clone().evaluate(*draw);
            if temp.clone().is_complete() 
            {
                let sum = temp.clone().sum_unmarked();

                //Sorry, I can't be arsed sorting this...
                if temp.clone().step_to_complete > 80 {
                    println!("Answer: {} {}, -> {:?} taking {} steps", 
                        sum, temp.clone().completing_number, 
                        sum * temp.clone().completing_number,
                        temp.clone().step_to_complete
                    );
                }

                break;
            }

            temp = temp.clone();
        }  
    }
}