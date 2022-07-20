mod parSolver;

use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::collections::HashSet;
use std::ptr::null;
use crate::parSolver::solveAllSoln;


fn showsSol(inputs: HashSet<Vec<Vec<u32>>>){
    for big in inputs{
        for line in big{
            println!("{:?}",line);
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let file = File::open("Test//map2.txt").unwrap();
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


    let mut books = HashSet::new();

    solveAllSoln(&mut grid, 0, 0, &mut books);
    println!("There are {} ways to solve this Sudoku", books.len());

    showsSol(books);



    Ok(()) //close the file reader
}

