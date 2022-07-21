mod parSolver;
mod apiSolver;
mod seqSolver;
use chashmap::CHashMap;
use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::ptr::null;
use crate::apiSolver::apiSolution;
use crate::parSolver::solveAllSoln;


fn showsSol(inputs: CHashMap<Vec<Vec<u32>>, u32>){
    for big in inputs{
        for line in big.0{
            println!("{:?}",line);
        }
        println!("");
    }
}

fn main() -> io::Result<()> {

    let paths:String = "Test//map2.txt".to_string();

    let file = File::open(paths.clone()).unwrap();


    let reader = BufReader::new(file);
    let mut grid = vec![];
    for line in reader.lines(){         //get line from the file
        let mut row = Vec::new();                    //create new vector
        for c in line.unwrap().chars(){                  //get each of the char in that line
            if c != ',' {
                let x = c.to_digit(10).unwrap();    //convert char to u32
                row.push(x);
            }
        }

        grid.push(row);                                         //add the row vector to the grid
    }


    let books = CHashMap::new();

    solveAllSoln(&mut grid, 0, 0, &books);
    println!("There are {} ways to solve this Sudoku", books.len());

    showsSol(books);
    //
    //apiSolution(paths.clone());

    Ok(()) //close the file reader
}

