use super::board::{Board, Mark, State};
use std::cmp;
use std::time::Instant;
pub struct OptimalAi;

impl OptimalAi {
    fn opposite(mark: Mark) -> Mark {
        if mark == Mark::Cross {
            Mark::Nought
        } else {
            Mark::Cross
        }
    }

    pub fn make_move(board: &mut Board, ai_mark: Mark) {
        let now = Instant::now();
        let mut best_score = -999;
        let mut best_index = 0;

        for index in board.get_free_cells() {
            board.set_cell(ai_mark, index);
            let score = OptimalAi::minimax(board, -999, 999, false, ai_mark);
            board.clear_cell(index);

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        board.set_cell(ai_mark, best_index);
        println!(
            "Took {} milliseconds taken to choose move!",
            now.elapsed().as_millis()
        );
    }

    pub fn minimax(
        board: &mut Board,
        mut alpha: i32,
        mut beta: i32,
        maximising: bool,
        ai_mark: Mark,
    ) -> i32 {
        let state = board.get_state();
        let opp_mark = OptimalAi::opposite(ai_mark);

        match state {
            State::Win(mark) => return if mark == ai_mark { 1 } else { -1 },
            State::Draw => return 0,
            State::Unfinished => (),
        };

        let mut final_score = if maximising { -999 } else { 999 };

        for index in board.get_free_cells() {
            board.set_cell(if maximising { ai_mark } else { opp_mark }, index);

            let score = OptimalAi::minimax(board, alpha, beta, !maximising, ai_mark);

            board.clear_cell(index);

            if maximising {
                final_score = cmp::max(final_score, score);
                alpha = cmp::max(alpha, final_score);

                if beta <= final_score {
                    break;
                }
            } else {
                final_score = cmp::min(final_score, score);
                beta = cmp::min(beta, final_score);

                if alpha >= final_score {
                    break;
                }
            }
        }
        final_score
    }
}
