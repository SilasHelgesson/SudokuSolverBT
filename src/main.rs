use std::sync::Mutex;

const DIMENSION_X : usize = 9;
const DIMENSION_Y : usize = 9;

static GAME_AREA: Mutex<[[u8; DIMENSION_X]; DIMENSION_Y]> = Mutex::new([
    [0,0,0,0,1,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,6,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [2,0,0,0,0,9,0,0,4],
    [0,0,9,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,3,0,0,0,0],
]);

fn get_valid_positions(board: &[[u8; DIMENSION_X]; DIMENSION_Y], x: usize, y: usize) -> Vec<u8> {
    let mut solutions = vec![1,2,3,4,5,6,7,8,9];

    // row
    for col in 0..DIMENSION_X {
        let v = board[y][col];
        if v != 0 {
            solutions.retain(|&n| n != v);
        }
    }

    // column
    for row in 0..DIMENSION_Y {
        let v = board[row][x];
        if v != 0 {
            solutions.retain(|&n| n != v);
        }
    }

    // box
    let box_x = (x / 3) * 3;
    let box_y = (y / 3) * 3;

    for i in 0..3 {
        for j in 0..3 {
            let v = board[box_y + i][box_x + j];
            if v != 0 {
                solutions.retain(|&n| n != v);
            }
        }
    }

    solutions
}

fn solve(board: &mut [[u8; DIMENSION_X]; DIMENSION_Y],x: usize, y: usize) -> bool {
    if x == DIMENSION_X {
        return solve(board,0,y+1);
    }
    if y == DIMENSION_Y {
        return true;
    }
    if board[y][x] != 0 {
        return solve(board,x+1,y);
    }
    let placements = get_valid_positions(&board,x,y);
    for placement in placements {
        board[y][x] = placement;
        if solve(board,x+1,y) {
            return true;
        }
    }
    board[y][x] = 0;
    return false;
} 

fn print_area(board: &[[u8; DIMENSION_X]; DIMENSION_Y]) {
    for y in 0..DIMENSION_Y {
        if y % 3 == 0 && y != 0 {
            println!("------+-------+------");
        }

        for x in 0..DIMENSION_X {
            if x % 3 == 0 && x != 0 {
                print!("| ");
            }

            if board[y][x] == 0 {
                print!(". ");
            } else {
                print!("{} ", board[y][x]);
            }
        }

        println!();
    }
}


fn main() {
    println!("Hello, world!");
    let mut board = GAME_AREA.lock().unwrap();
    if solve(&mut board, 0, 0) {
        println!("Solution found:");
        print_area(&board);
    } else {
        println!("No solution exists.");
    }
}
