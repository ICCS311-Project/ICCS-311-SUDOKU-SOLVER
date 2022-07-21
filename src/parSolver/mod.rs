use core::num;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io;
use std::io::*;
use chashmap::CHashMap;
use std::ptr::null;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};



fn par_sum(v: &[i32]) -> i32 {
    if v.len() <= 1 { //
    return v.iter().sum();
    }
    let (left, right) = v.split_at(v.len()/2);
    let (left_sum, right_sum) = rayon::join(|| par_sum(left),
    || par_sum(right));
    left_sum + right_sum
}

#[allow(dead_code)]
fn col_check<u32: Copy + Send + Ord + Sync>(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
    let size = grid[0].len();
    for j in 0..size{
        if grid[j][col] == num{             //if the number already exists within that column then return false
            return true
        }
    }
    false

}

//if the number already exists within that row then return false
fn row_check(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{

    if grid[row].contains(&num){            
        return true
    }
    false

}

fn is_safe(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{

    let(horizontal, vertical) = rayon::join(||row_check(grid, num, row, col), || col_check(grid, num, row, col));

    if (horizontal || vertical){
        return false
    }

    let size = grid[0].len();       //grid size i.e. 9
    let rowChop = row - row%3;      //3*3 block
    let colChop = col - col%3;
    // e.g. row = 2 col = 5
    // rowchop = 2-2 = 0 colchop = 5 - 2 = 3
    // then the search block would be block [0,1]
    // and the search cells would be row 0-> 2 and col 3->5

    let mut i = rowChop;
    while i <= rowChop+2{
        let mut j = colChop;
        while j <= colChop+2{
            if grid[i][j] == num {    //if the number already exists within that block then return false
                return false
            }
            j+=1;
        }
        i +=1;
    }

    return true;
}

pub fn solveAllSoln(grid: &mut Vec<Vec<u32>>, mut row: usize, mut col: usize,  resultSet: & CHashMap<Vec<Vec<u32>>, u32> ){
    let grid_size: usize = grid.len();
    // base case check if we reach the last cell i.e. row = 8 and col = 8
    if row == grid_size-1 && col == grid_size-1 { //grid_size = 8 for 9*9


        if grid[row][col] > 0 {     //if the cell in the last index has valid number then we save it to our resultSet
            resultSet.insert(grid.clone(),0);
            return ();
        }
        else {                      //when the last cell is blank then we check for possible num to be in the cell
            (1..=9).into_iter().for_each(|num| {
                if is_safe(grid, num, row, col){
                    grid[row][col] = num;
                    resultSet.insert(grid.clone(),0);

                    grid[row][col] = 0; // reset the grid[row][col] to 0 (unfilled) so, that we can search for other possible soln
                }
            }
        )

        }
        return ();
    }


    if col == grid_size {

        return solveAllSoln(grid, row+1, 0, resultSet);
    }
    if grid[row][col] == 0{    //if cell is empty then check for all possible number
       
        (1..=9).into_par_iter().for_each(|num| {

            if is_safe(grid, num, row, col){
                let mut newgrid = grid.clone();
                newgrid[row][col] = num;


                solveAllSoln(&mut newgrid, row, col+1, resultSet);
                newgrid[row][col] = 0;
            }
        });
    }
     
    else {
        solveAllSoln(grid, row, col+1, resultSet);
    }
    return ();

}


