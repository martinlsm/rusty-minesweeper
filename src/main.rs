use std::io;

mod board;

fn main() {
    let mut board = board::new(9, 9, 10).unwrap();
    loop {
        let mut user_input = String::new();
        if let Err(_) = io::stdin().read_line(&mut user_input) {
            println!("Could not read user input");
            break;
        }

        let tile: Vec<usize> = user_input
            .trim()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .take(2)
            .collect();

        board.reveal(tile[0], tile[1]);
        board.debug_print();
    }
}
