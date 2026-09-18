/*
You are given row x col grid representing a map where grid[i][j] = 1 represents
land and grid[i][j] = 0 represents water.
Grid cells are connected horizontally/vertically (not diagonally).
The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
The island doesn't have "lakes", meaning the water inside isn't connected to the water
around the island. One cell is a square with side length 1.
The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.

Example 1:
Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
Output: 16
Explanation: The perimeter is the 16 yellow stripes in the image above.

Example 2:
Input: grid = [[1]]
Output: 4

Example 3:
Input: grid = [[1,0]]
Output: 4
*/

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    let res = island_perimeter(grid);
    println!("{}", res);
    assert!(16 == res);
    println!("passed");

    let grid = vec![vec![1]];
    let res = island_perimeter(grid);
    println!("{}", res);
    assert!(4 == res);
    println!("passed");

    let grid = vec![vec![1, 0]];
    let res = island_perimeter(grid);
    println!("{}", res);
    assert!(4 == res);
    println!("passed")
}

pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                return dfs(&mut grid, i as i32, j as i32);
            }
        }
    }

    return 0;
}

fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    if i < 0 || j < 0 || i >= rows || j >= cols || grid[i as usize][j as usize] == 0 {
        return 1;
    }

    if grid[i as usize][j as usize] == -1 {
        return 0;
    }

    grid[i as usize][j as usize] = -1;

    return dfs(grid, i - 1, j) + dfs(grid, i + 1, j) + dfs(grid, i, j - 1) + dfs(grid, i, j + 1);
}
