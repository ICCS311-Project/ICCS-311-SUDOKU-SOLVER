use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::collections::HashSet;
use std::ptr::null;
use rayon::prelude::*;

fn is_safe(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
    let size = grid[0].len();       //grid size i.e. 9

    if grid[row].contains(&num){            //if the number already exists within that row then return false
        return false
    }

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

                    resultSet.insert(grid.to_owned());

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
        for num in 1..=9 {
            if is_safe(grid, num, row, col){
                grid[row][col] = num;

                solveAllSoln(grid, row, col+1, resultSet);
                grid[row][col] = 0;
            }
        }
    }
    else {
        solveAllSoln(grid, row, col+1, resultSet);
    }
    return resultSet;

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

