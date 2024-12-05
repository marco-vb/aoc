use std::{fs, vec};

fn search(i: i32, j: i32, g: &Vec<&[u8]>) -> i32 {
    let mut ret = 0;

    let r = g.len() as i32;
    let c = g.first().unwrap().len() as i32;

    let p: &[u8] = b"XMAS";
    let dirs = [(0, 1), (1, 0), (1, 1), (-1, 1)];

    for dir in dirs {
        for sign in [-1, 1] {
            let ix = dir.0 * sign;
            let iy = dir.1 * sign;

            for inc in 0..4 {
                let x = i + inc * ix;
                let y = j + inc * iy;

                if x < 0 || x >= r || y < 0 || y >= c {
                    break;
                }

                if p[inc as usize] != g[x as usize][y as usize] {
                    break;
                }

                if inc == 3 {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn first_task(grid: &Vec<&[u8]>) -> i32 {
    let r = grid.len();
    let c = grid.first().unwrap().len();

    let mut s = 0;
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == b'X' {
                s += search(i as i32, j as i32, &grid);
            }
        }
    }
    s
}

fn search_cross(i: i32, j: i32, g: &Vec<&[u8]>) -> i32 {
    let mut ret = 0;
    let dirs = [
        [(-1, 1), (-1, -1)],
        [(-1, 1), (1, 1)],
        [(1, 1), (1, -1)],
        [(1, -1), (-1, -1)],
    ];

    for dir in dirs {
        let lx = dir[0].0 + i;
        let ly = dir[0].1 + j;
        let rx = dir[1].0 + i;
        let ry = dir[1].1 + j;

        if g[lx as usize][ly as usize] != b'M' || g[rx as usize][ry as usize] != b'M' {
            continue;
        }

        let lx = i - dir[0].0;
        let ly = j - dir[0].1;
        let rx = i - dir[1].0;
        let ry = j - dir[1].1;

        if g[lx as usize][ly as usize] != b'S' || g[rx as usize][ry as usize] != b'S' {
            continue;
        }

        ret += 1;
    }
    ret
}

fn second_task(grid: &Vec<&[u8]>) -> i32 {
    let r = grid.len();
    let c = grid.first().unwrap().len();

    let mut s = 0;
    for i in 1..r - 1 {
        for j in 1..c - 1 {
            if grid[i][j] == b'A' {
                s += search_cross(i as i32, j as i32, &grid);
            }
        }
    }
    s
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<&[u8]> = vec![];

    for line in contents.lines() {
        grid.push(line.as_bytes());
    }

    let s1 = first_task(&grid);
    let s2 = second_task(&grid);

    println!("Task 1: {s1}");
    println!("Task 2: {s2}");
}
