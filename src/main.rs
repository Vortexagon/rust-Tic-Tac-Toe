mod board;
mod ai;

use board::{Board, Layer, State};
use ai::OptimalAi;

use std::io::{self, Write};

fn handle_user_move(board: &mut Board, layer: &Layer) {
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

        board.set_cell(layer, index);
        break;
    }
}

fn main() {
    let mut game_board = Board::new();

    while game_board.get_state() == State::Unfinished {

        game_board.print_board();
        handle_user_move(&mut game_board, &Layer::X);

        if game_board.get_state() != State::Unfinished {
            break;
        };

        OptimalAi::make_move(&mut game_board, &Layer::O);
    }

    game_board.print_board();
    println!("Result: {:?}", game_board.get_state());

}
