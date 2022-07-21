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

fn showSolHS(inputs: HashSet<Vec<Vec<u32>>>){
    for big in inputs{
        for line in big{
            println!("{:?}",line);
        }
        println!("");
    }
}


fn runSeqSolver(grid: &mut Vec<Vec<u32>>) -> (Duration, usize){
    println!("running sequential sudoku solver");
    let mut solutions :HashSet<Vec<Vec<u32>>> = HashSet::new();
    let now = Instant::now();
    {
    seqSolver::solveAllSoln(grid, 0, 0, &mut solutions);
    }
    let elapsed = now.elapsed();
    showSolHS(solutions.clone());
    return (elapsed as Duration, solutions.len());
}

fn runParSolver(grid: &mut Vec<Vec<u32>>) -> (Duration, usize){
    println!("running parallel sudoku solver");
    let solutions = CHashMap::new();
    let now = Instant::now();
    {
    parSolver::solveAllSoln(grid, 0, 0, &solutions);
    }
    let elapsed = now.elapsed();
    showSol(solutions.clone());
    return (elapsed as Duration, solutions.len());
}

fn runApiSolver(path: String) -> (Duration, usize){
    println!("running Rust api sudoku solver");
    let mut a: u32 = 0;
    let now = Instant::now();
    {
    a = apiSolver::apiSolution(path.to_string());
    }
    let elapsed = now.elapsed();
    return (elapsed as Duration, a.try_into().unwrap());
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

    let (parTime, noSolPar) = runParSolver(&mut grid);
    let (seqTime, noSolSeq) = runSeqSolver(&mut grid);
    let (apiTime, noSolApi) = runApiSolver(path.to_string());
    println!("Parallel sudoku solver has {} solutions and it takes: {:.2?} :P", noSolPar,parTime);
    println!("Sequential sudoku solver has {} solutions and it takes: {:.2?} :P", noSolSeq,seqTime);
    println!("Api sudoku solver has {} solutions and it takes: {:.2?} :P", noSolApi,apiTime);
    Ok(()) //close the file reader
}

