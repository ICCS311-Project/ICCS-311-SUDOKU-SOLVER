use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use sudoku_solver::{Board, SolutionIter, solve};

pub(crate) fn apiSolution(path: String){

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut grid = [[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0]];
    let mut indexs = 0;
    for line in reader.lines(){
        let mut row = [0,0,0,0,0,0,0,0,0];
        let mut index = 0;
        for c in line.unwrap().chars(){
            if index >9{
                index = 0;
            }
            if c != ',' {
                let x = c.to_digit(10).unwrap() as u8;
                row[index] = x;
                index += 1;
            }
        }

        grid[indexs] = row;
        indexs += 1;
    }   let board = Board::from(&grid);

    let mut count = 0;
    let mut solutions = SolutionIter::new(&board);

    for x in solutions.into_iter() {

        println!("{}",x);
        println!("");
        count += 1;
    }

    println!("{}",count);


}