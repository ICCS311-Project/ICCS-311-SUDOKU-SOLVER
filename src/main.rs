use std::fs::File;
use std::io;
use std::io::*;



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


fn solveGrid(grid: &mut Vec<Vec<u32>>, mut row: usize, mut col: usize) -> bool{
    let size = grid.len();
    if row == size-1 && col == size{                //check if there is any possible move in the grid left i.e. if its the last row and last col if yes then return true
        return true
    }
    if col == size{                                 //if the col in that row reached the size then row += 1 and col becomes 0 again
        return solveGrid(grid, row + 1, 0); 
    }
    if grid[row][col] > 0{                          //check if the cell is empty -> if it is then move to the next col
        return solveGrid(grid, row, col + 1);       
    }

    for num in 1..=9 {
        if is_safe(&grid, num, row, col){           //check if that number is possible  if it is then insert that number
            grid[row][col] = num;

            if solveGrid(grid, row, col + 1){       // check the next possibility with the next col
                return true;
            }

        }
        grid[row][col] = 0;                         //if it's not safe/possible then let the grid's cell remain 0
    }
    return false;
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

    let mut newGrid = grid.clone();// Copy the old grid to solve it and another will be the unsolve. And use it to compare
    // grid sudoku <= unsolve
    //newGrid sudoku <= solved


    if solveGrid(&mut newGrid,0,0){
        for line in newGrid{
            for cell in line{
                print!("{}, ", cell);
            }
            println!("");
        }

    }
    println!("");

    for line in grid{
        for cell in line{
            print!("{}, ", cell);
        }
        println!("");
    }

    Ok(()) //close the file reader
}

