use core::num;
use std::fs::File;
use std::io;
use std::io::*;
use std::collections::HashSet;
use std::ptr::null;

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

    let mut i = 0;
    let mut j = 0;
    while i < 3{
        while j < 3{
            if grid[i+rowChop][j+colChop] == num {    //if the number already exists within that block then return false
                return false
            }
            j+=1;
        }
        i +=1;
    }

    return true;
}


// fn solveGrid(grid: &mut Vec<Vec<u32>>, mut row: usize, mut col: usize) -> bool{
//     let size = grid.len();
//     if row == size-1 && col == size{                //check if there is any possible move in the grid left i.e. if its the last row and last col if yes then return true
//         return true
//     }
//     if col == size{                                 //if the col in that row reached the size then row += 1 and col becomes 0 again
//         return solveGrid(grid, row + 1, 0);
//     }
//     if grid[row][col] > 0{                          //check if the cell is empty -> if it is then move to the next col
//         return solveGrid(grid, row, col + 1);
//     }
//
//     for num in 1..=9 {
//         if is_safe(&grid, num, row, col){           //check if that number is possible  if it is then insert that number
//             grid[row][col] = num;
//
//             if solveGrid(grid, row, col + 1){       // check the next possibility with the next col
//                 return true;
//             }
//
//         }
//         grid[row][col] = 0;                         //if it's not safe/possible then let the grid's cell remain 0
//     }
//     return false;
// }

fn solveAllSoln<'a>(grid: &'a mut Vec<Vec<u32>>, mut row: usize, mut col: usize, mut resultSet: &'a mut HashSet<Vec<Vec<u32>>> ) -> &'a HashSet<Vec<Vec<u32>>> {

    let grid_size: usize = grid.len();
    // base case check if we reach the last cell i.e. row = 8 and col = 8
    if row == grid_size-1 && col == grid_size { //grid_size = 8 for 9*9

        if grid[row][col-1] > 0 {     //if the cell in the last index has valid number then we save it to our resultSet
            // println!("WORK {}, {}", row, col-1);
            resultSet.insert(grid.clone());
            // println!("{}", resultSet.len());
            return resultSet;
        }
        else {                      //when the last cell is blank then we check for possible num to be in the cell
            for num in 1..=9 {
                if is_safe(grid, num, row, col){
                    grid[row][col] = num;

                    resultSet.insert(grid.to_owned());

                    grid[row][col] = 0; // reset the grid[row][col] to 0 (unfilled) so, that we can search for other possible soln
                }
            }

        }
        return resultSet;
    }


    if col == grid_size {
        //solveAllSoln(grid, row+1, 0, resultSet);
        return solveAllSoln(grid, row+1, 0, resultSet);
    }
    if grid[row][col] == 0 {    //if cell is empty then check for all possible number
        for num in 1..=9 {
            if is_safe(grid, num, row, col){
                grid[row][col] = num;
                //println!("UPDATED {} at row {} col {}", grid[row][col], row, col);

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

// fn recursiveLoop(mut start: u32, mut stop: u32) -> u32 {
//     match start {
//         stop => start,
//         _ => {println!("hi{}",start);
//             recursiveLoop(start+1, stop)},
//     }
// }

// fn recursiveLoop2(mut start: u32, mut stop: u32) -> u32 {
//     if start == stop+1 {return start}
//     else{
//         println!("hello {}", start);
//         recursiveLoop2(start+1, stop)
//     }
// }

fn printDeezNuts(inputs: HashSet<Vec<Vec<u32>>>){
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
    let file = File::open("Test//map1.txt").unwrap();
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

    //let mut newGrid = grid.clone();// Copy the old grid to solve it and another will be the unsolve. And use it to compare
    // grid sudoku <= unsolve
    //newGrid sudoku <= solved


    // if solveGrid(&mut newGrid,0,0){
    //     for line in newGrid{
    //         for cell in line{
    //             print!("{}, ", cell);
    //         }
    //         println!("");
    //     }

    // }
    // println!("");

    // for line in grid{
    //     for cell in line{
    //         print!("{}, ", cell);
    //     }
    //     println!("");
    // }

    //let allSoln: HashSet<Vec<Vec<u32>>> = solveAllSoln(&mut grid, 0, 0, HashSet::new());

    // let mut grid1 = vec![
    // vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    // vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    // vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    // vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    // vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    // vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    // vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    // vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    // vec![0, 0, 0, 0, 8, 0, 0, 0, 0],
    // ];
    //
    // let mut grid2 = vec![
    //     vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     vec![0, 0, 0, 0, 8, 0, 0, 0, 0],
    //     ];
    //
    //     let mut grid3 = vec![
    //     vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     vec![0, 0, 0, 0, 8, 0, 0, 0, 1],
    //     ];

    let mut books = HashSet::new();
    //let result =
    solveAllSoln(&mut grid, 0, 0, &mut books);
    println!("{}", books.len());

    printDeezNuts(books);
    // books.insert(grid1);
    // books.insert(grid2);
    // books.insert(grid3);


    //recursiveLoop2(0, 4);
    // for n in 1..=10 {
    //     println!("{}",n);
    // }


    Ok(()) //close the file reader
}

