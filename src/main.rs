use std::fs::File;
use std::io;
use std::io::*;

fn is_save(grid: Vec<Vec<u64>>, num: u64, row: usize, col: usize) -> bool{
    let size = grid[0].len();
    for i in size{
        if grid[row][i] == num{
            return false
        }
    }

    for j in size{
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
            if grid[i+rowChop][j+colChop] == num{
                return false
            }
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("Test//map1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid = vec![];
    for line in reader.lines(){
        let mut row = Vec::new();
        for c in line.unwrap().chars(){
            if c != ',' {
                row.push(c);
            }
        }

        grid.push(row);
    }


    Ok(())
}
