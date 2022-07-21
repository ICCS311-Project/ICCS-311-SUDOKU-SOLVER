mod parSolver;
mod apiSolver;
mod seqSolver;
use chashmap::CHashMap;
use core::num;
use std::{fs::File, collections::HashSet};
use std::io;
use std::io::*;
use std::ptr::null;
use crate::apiSolver::apiSolution;
use crate::parSolver::solveAllSoln;



use std::time::{Duration, Instant};

fn showSol(inputs: CHashMap<Vec<Vec<u32>>, u32>){
    for big in inputs{
        for line in big.0{
            println!("{:?}",line);
        }
        println!("");
    }
}


fn runSeqSolver(grid: &mut Vec<Vec<u32>>) -> Duration{
    let now = Instant::now();
    {
    let mut solutions :HashSet<Vec<Vec<u32>>> = HashSet::new();
    seqSolver::solveAllSoln(grid, 0, 0, &mut solutions);
    println!("Running Seq Solver. There are {} ways to solve this Sudoku", solutions.len());
    // showSol(solutions);
    }
    let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    return elapsed as Duration
}

fn runParSolver(grid: &mut Vec<Vec<u32>>) -> Duration{
    let now = Instant::now();
    {
    let solutions = CHashMap::new();
    parSolver::solveAllSoln(grid, 0, 0, &solutions);
    println!("There are {} ways to solve this Sudoku", solutions.len());
    //showSol(books);
    }
    let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);
    return elapsed as Duration
}





fn main() -> io::Result<()> {

    let path = "Test//map2.txt";
    
    let file = File::open(path).unwrap();
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

    
    // let books = CHashMap::new();

    // parSolver::solveAllSoln(&mut grid, 0, 0, &books);
    // println!("There are {} ways to solve this Sudoku", books.len());

    // showSol(books);
    runParSolver(&mut grid);
    //runSeqSolver(&mut grid);
    apiSolver::apiSolution(path.to_string());
    
    
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        let mut solutions :HashSet<Vec<Vec<u32>>> = HashSet::new();
        seqSolver::solveAllSoln(&mut grid, 0, 0, &mut solutions);
        println!("Running Seq Solver. There are {} ways to solve this Sudoku", solutions.len());
        showsSol(solutions);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // let now = Instant::now();

    // // Code block to measure.
    // {
    //     let mut parSolutions = HashSet::new();
    //     parSolver::solveAllSoln(&mut grid, 0, 0, &mut parSolutions);
    //     println!("Running Party Solver. There are {} ways to solve this Sudoku", parSolutions.len());
    //     showsSol(parSolutions);
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);



    Ok(()) //close the file reader
}

