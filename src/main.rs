mod board;
mod ai;

use board::{Board, Mark, State};
use ai::OptimalAi;

use std::io::{self, Write};

fn handle_user_move(board: &mut Board, mark: &Mark) {
    loop {
        let mut choice = String::new();

        print!("Enter a number: ");
        io::stdout().flush().expect("Error flushing output");

        io::stdin()
            .read_line(&mut choice)
            .expect("Error handling input");

        let index: u32 = match choice.trim().parse() {
            Ok(num) => {
                if num <= 8 && board.get_free_cells().contains(&num) {
                    num
                } else {
                    continue
                }
            },
            Err(_) => continue,
        };

        board.set_cell(mark, index);
        break;
    }
}

fn main() {
    let mut game_board = Board::default();

    while game_board.get_state() == State::Unfinished {

        println!("{}", game_board);
        handle_user_move(&mut game_board, &Mark::Cross);

        if game_board.get_state() != State::Unfinished {
            break;
        };

        OptimalAi::make_move(&mut game_board, &Mark::Nought);
    }

    println!("{}", game_board);
    println!("Result: {:?}", game_board.get_state());

}
