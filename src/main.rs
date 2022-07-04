use std::fs::File;
use std::io;
use std::io::*;


fn is_save(mut grid: &Vec<Vec<u64>>, num: u64, row: usize, col: usize) -> bool{
    let size = grid[0].len();
    for i in 0..=size{
        if grid[row][i] == num{
            return false
        }
    }

    for j in 0..=size{
        if grid[j][col] == num{
            return false
        }
    }

    let rowChop = row - row%3;
    let colChop = col - col%3;
    let mut i = 0;
    let mut j = 0;
    while i < 3{
        while j < 3{
            if grid[i+rowChop][j+colChop] == num {
                return false
            }
        }
    }

    return true;
}


fn solveGrid(grid: &mut Vec<Vec<u64>>, mut row: usize, mut col: usize) -> bool{
    let size = grid.len();
    if row == size-1 && col == size{
        return true
    }
    if col == size{
        return solveGrid(grid, row + 1, 0);
    }

    if grid[row][col] != 0{
        return solveGrid(grid, row, col + 1);
    }

    for num in 1..=9 {
        if is_save(&grid, num, row, col) == true{
            grid[row][col] = num;

            if solveGrid(grid, row, col + 1){
                return true;
            }
            grid[row][col] = 0;
        }
    }
    return false;
}


fn main() -> io::Result<()> {
    let file = File::open("Test//map1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid = vec![];
    for line in reader.lines(){
        let mut row = Vec::new();
        for c in line.unwrap().chars(){
            if c != ',' {
                row.push(c as u64);
            }
        }

        grid.push(row);
    }


    solveGrid(&mut grid, 0, 0);

    println!("{}",grid[0][1]);

    Ok(())
}
