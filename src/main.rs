use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::collections::HashSet;
use std::ptr::null;
use rayon::prelude::*;

mod parSolver;
mod seqSolver;
use std::time::{Duration, Instant};

fn timed<R, F>(mut f: F) -> (R, Duration) where F: FnMut() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
    }

fn showsSol(inputs: HashSet<Vec<Vec<u32>>>){
    for big in inputs{
        for line in big{
            for cell in line{
                print!("{}, ", cell);
            }
            println!("");
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

    let mut solutions = HashSet::new();
    seqSolver::solveAllSoln(&mut grid, 0, 0, &mut solutions);
    println!("Running Seq Solver. There are {} ways to solve this Sudoku", solutions.len());
    showsSol(solutions);

    let mut parSolutions = HashSet::new();
    parSolver::solveAllSoln(&mut grid, 0, 0, &mut parSolutions);
    println!("Running Party Solver. There are {} ways to solve this Sudoku", parSolutions.len());
    showsSol(parSolutions);

    // let mut solutions = HashSet::new();
    // let (solutions, t) = timed(|| seqSolver::solveAllSoln(&mut grid, 0, 0, &mut solutions));
    // println!("msort: sorted t={}s", t.as_secs_f64());
    // showsSol(solutions.clone());

    // let mut parSolutions = HashSet::new();
    // let (parSolutions, t) = timed(|| parSolver::solveAllSoln(&mut grid, 0, 0, &mut parSolutions));
    // println!("par_msort_simple: sorted t={}s", t.as_secs_f64());
    // showsSol(parSolutions.clone());
    // let (sorted, t) = timed(|| par_msort(&xs));
    // println!("par_msort: sorted={:?}, t={}s", is_sorted(&sorted), t.as_secs_f64());

    Ok(()) //close the file reader
}

