use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::collections::HashSet;
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
    // let size = grid[0].len();
    // let (up, down) = (0..size/2, size/2..size);
    // let (left_sum, right_sum) = rayon::join(|| par_sum(left),
    // || par_sum(right));
    // left_sum + right_sum


    // fn helper(mut grid: &Vec<Vec<u32>>, num: u32, rows: &Vec<usize>, col: usize){
    //     let magic_number = 1;

    //     if rows.len() <= magic_number {
            
    // }
    // if row.len() <= 1 {
    //     if grid[row[0]][col] == num {return false}
    //     else {return true}
    // }
    // else{
    //     if grid[row[0]][col] == num {return false}
    //     row.pop();
    // }    
    // let size = grid[0].len();
    // let (up, down) = (0..size/2, size/2..size);
    // let (up_sum, down_sum) = rayon::join(|| par_sum(up),
    // || par_sum(down));
    // up_sum && down_sum
    true
}

//if the number already exists within that row then return false
fn row_check<u32: Copy + Send + Ord + Sync>(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
    let size = grid[0].len();
    let (left, right) = grid[row].split_at(size/2);
    let (left_sum, right_sum) = rayon::join(|| left.contains(&num),
    || right.contains(&num));
    if left_sum || right_sum{
        return true;
    }
    else{
        return false
    }

}

fn is_safe<u32: Copy + Send + Ord + Sync>(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
    let size = grid[0].len();       //grid size i.e. 9

    //row check
    // if grid[row].contains(&num){            
    //     return false
    // }
    if row_check(grid, num, row, col){ return false}

    //col check
    for j in 0..size{
        if grid[j][col] == num{             //if the number already exists within that column then return false
            return false
        }
    }
    


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

pub fn solveAllSoln<'a>(grid: &'a mut Vec<Vec<u32>>, mut row: usize, mut col: usize, mut resultSet: &'a mut HashSet<Vec<Vec<u32>>> ) -> &'a HashSet<Vec<Vec<u32>>> {

    let grid_size: usize = grid.len();
    // base case check if we reach the last cell i.e. row = 8 and col = 8
    if row == grid_size-1 && col == grid_size-1 { //grid_size = 8 for 9*9

        if grid[row][col] > 0 {     //if the cell in the last index has valid number then we save it to our resultSet

            resultSet.insert(grid.clone());

            return resultSet;
        }
        else {                      //when the last cell is blank then we check for possible num to be in the cell
            (1..=9).into_iter().for_each(|num| {
                if is_safe(grid, num, row, col){
                    grid[row][col] = num;

                    resultSet.insert(grid.clone());

                    grid[row][col] = 0; // reset the grid[row][col] to 0 (unfilled) so, that we can search for other possible soln
                }
            }
        )

        }
        return resultSet;
    }


    if col == grid_size {
        return solveAllSoln(grid, row+1, 0, resultSet);
    }
    if grid[row][col] == 0 {    //if cell is empty then check for all possible number
        (1..=9).into_iter().for_each(|num| {
            if is_safe(grid, num, row, col){
                grid[row][col] = num;

                solveAllSoln(grid, row, col+1, resultSet);
                grid[row][col] = 0;
            }
        })
    }
    else {
        solveAllSoln(grid, row, col+1, resultSet);
    }
    return resultSet;

}


