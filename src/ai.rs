use super::board::{Board, State, Mark};

pub struct OptimalAi;

impl OptimalAi {
    fn opposite(mark: &Mark) -> &Mark {
        if mark == &Mark::Cross { &Mark::Nought } else { &Mark::Cross }
    }

    pub fn make_move(board: &mut Board, ai_mark: &Mark) {
        let mut best_score = -999;
        let mut best_index = 0;

        for index in board.get_free_cells() {
            board.set_cell(ai_mark, index);
            let score = OptimalAi::minimax(board, false, ai_mark);
            board.clear_cell(index);

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        board.set_cell(ai_mark, best_index);
    }

    pub fn minimax(board: &mut Board, maximising: bool, ai_mark: &Mark) -> i32 {
        let state = board.get_state();
        let opp_mark = OptimalAi::opposite(ai_mark);

        match state {
            State::Win(mark) => {
                if mark == *ai_mark { return 1; }
                else { return -1; }
            },
            State::Draw => return 0,
            State::Unfinished => {},
        };

        let mut final_score = if maximising { -999 } else { 999 };

        for index in board.get_free_cells() {
            board.set_cell(if maximising {ai_mark} else {opp_mark}, index);
            let score = OptimalAi::minimax(board, !maximising, ai_mark);
            board.clear_cell(index);

            if maximising && score > final_score || !maximising && score < final_score {
                final_score = score;
            }
        }
        final_score
    }
}
