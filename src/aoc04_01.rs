use std::io::stdin;

pub fn fn04_01() {
    let mut board: Vec<Vec<u8>> = Vec::new();
    stdin().lines().for_each(|line| {
        let line = line.unwrap();
        board.push(line.as_bytes().to_vec());
    });

    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let target: [u8; 4] = [b'X', b'M', b'A', b'S'];

    let result = board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    directions
                        .iter()
                        .filter(|direction| search(&board, (y, x), &target, **direction))
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{}", result);
}

fn search(
    board: &Vec<Vec<u8>>,
    cursor: (usize, usize),
    target: &[u8],
    direction: (i32, i32),
) -> bool {
    if board[cursor.0][cursor.1] != target[0] {
        return false;
    }

    let next_target = &target[1..];
    if next_target.is_empty() {
        return true;
    }

    let may_be_next_y = cursor.0 as i32 + direction.0;
    let may_be_next_x = cursor.1 as i32 + direction.1;

    if may_be_next_y >= board.len() as i32 || may_be_next_y < 0 {
        return false;
    }
    let next_y = may_be_next_y as usize;

    if may_be_next_x >= board[next_y].len() as i32 || may_be_next_x < 0 {
        return false;
    }
    let next_x = may_be_next_x as usize;

    let next_cursor = (next_y, next_x);

    search(board, next_cursor, next_target, direction)
}
