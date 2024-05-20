use std::io::{prelude::*, BufWriter};

use rand::{Rng, prelude::*};
use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};

use crate::resources::Board;

fn send_solve(solve: Vec<String>) {
    let name = "number-shifting.sock".to_ns_name::<GenericNamespaced>().unwrap();

    let mut conn = BufWriter::new(Stream::connect(name).unwrap());

    conn.write_all(
        (solve.join("\n")+"\n").as_bytes()
    ).unwrap();
}

pub fn generate_level(level: usize) -> Board {
    let mut rng = thread_rng();
    
    let mut spawns = 3 + if level > 150 {
        level - 75
    } else {
        level / 2
    };
    
    let mut height = 5;
    let mut width = height * 16 / 9;
    
    while width * height < spawns * 2 {
        spawns -= 2;
        height += 1;
        width = height * 16 / 9;
    }
    
    let mut board = Board::new(width, height);
    let mut solve = Vec::new();

    for i in 0..spawns {
        let gen_pair = i == 0 || rng.gen_range(0..5) == 0;
        loop {
            let x1 = rng.gen_range(0..width);
            let y1 = rng.gen_range(0..height);

            let dir = rng.gen_range(0..4); // URDL
            let length_range = match dir {
                0 => 1..height-y1,
                1 => 1..width-x1,
                2 => 1..y1+1,
                3 => 1..x1+1,
                _ => unreachable!()
            };
            if length_range.is_empty() { continue; }
            let length = rng.gen_range(length_range);
            let x2 = match dir {
                0 => x1,
                1 => x1 + length,
                2 => x1,
                3 => x1 - length,
                _ => unreachable!()
            };
            let y2 = match dir {
                0 => y1 + length,
                1 => y1,
                2 => y1 - length,
                3 => y1,
                _ => unreachable!()
            };
            
            if gen_pair {
                if board.grid[y1][x1] == 0 && board.grid[y2][x2] == 0 {
                    board.grid[y1][x1] = length;
                    board.grid[y2][x2] = length;
                    solve.push(format!(
                        "{x1} {y1} {} -",
                        "URDL".get(dir..=dir).unwrap()
                    ));
                    break;
                }
            } else {
                if board.grid[y1][x1] != 0 && board.grid[y1][x1] != length && board.grid[y2][x2] == 0 {
                    board.grid[y2][x2] = length;

                    let mut add = rng.gen_bool(0.5);
                    if !add {
                        if board.grid[y1][x1] < length { add = !add }
                        board.grid[y1][x1] = (
                            board.grid[y1][x1] as i32 - length as i32
                        ).abs() as usize;
                    }
                    else { board.grid[y1][x1] += length; }
                    let solve_dir = (dir + 2) % 4;
                    solve.push(format!(
                        "{x2} {y2} {} {}",
                        "URDL".get(solve_dir..=solve_dir).unwrap(),
                        if add { '-' } else { '+' }
                    ));

                    break;
                }
            }
        }
    }

    send_solve(solve);

    board
}
