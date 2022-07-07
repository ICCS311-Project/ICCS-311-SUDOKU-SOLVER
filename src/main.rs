use std::fs::File;
use std::io;
use std::io::*;
use array2d::Array2D;
use board::*;
use sudoku_solver::*;


fn is_save(mut grid: &Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> bool{
    let size = grid[0].len();
    let mut error = 0;
    if grid[row].contains(&num){
        return false

    }

    for j in 0..size{
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
            j+=1;
        }
        i +=1;
    }

    return true;
}


fn solveGrid(grid: &mut Vec<Vec<u32>>, mut row: usize, mut col: usize) -> bool{
    let size = grid.len();
    if row == size-1 && col == size{
        return true
    }
    if col == size{
        return solveGrid(grid, row + 1, 0);
    }

    if grid[row][col] > 0{

        return solveGrid(grid, row, col + 1);
    }
    let mut time = 0;
    // while time < 20{
    //
    // //loop{
    //     let mut rng = rand::thread_rng();
    //     let num = rng.gen_range(1, 10);

    for num in 1..=9 {
        if is_save(&grid, num, row, col) {
            grid[row][col] = num;
            println!("{} put at {} {}", num, row, col);
            println!("");
            if solveGrid(grid, row,col + 1) {
                return true
            }
        }

        grid[row][col] = 0;
    }
    return false;
}


fn main() -> io::Result<()> {
    let file = File::open("Test//map2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid = [[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0]];
    let mut indexs = 0;
    for line in reader.lines(){
        let mut row = [0,0,0,0,0,0,0,0,0];
        let mut index = 0;
        for c in line.unwrap().chars(){
            if index >9{
                index = 0;
            }
            if c != ',' {
                let x = c.to_digit(10).unwrap() as u8;
                row[index] = x;
                index += 1;
            }
        }

        grid[indexs] = row;
        indexs += 1;
    }







    let board = Board::from(&grid);

    let mut solutions = SolutionIter::new(&board);

    // for x in solutions.into_iter() {
    //
    //     println!("{}",x);
    //     println!("");
    // }

    println!("{}",solutions.count());

    if let Some(solution) = solve(&board) {
        println!("Solution:\n{}\n", solution);
    } else {
        println!("No solution found.");
    }

    // let mut newGrid = grid.clone();
    //
    //
    //
    // if solveGrid(&mut newGrid,0,0){
    //     for lel in newGrid{
    //         for olo in lel{
    //             print!("{}, ", olo);
    //         }
    //         println!("");
    //     }
    //
    // }
    // println!("");





    Ok(())
}
