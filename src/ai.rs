use super::board::{Board, State, Layer};

pub struct OptimalAi;

impl OptimalAi {
    fn opposite(layer: &Layer) -> &Layer {
        if layer == &Layer::X { &Layer::O } else { &Layer::X }
    }

    pub fn make_move(board: &mut Board, ai_layer: &Layer) {
        let mut best_score = -999;
        let mut best_index = 0;

        for index in board.get_free_cells() {
            board.set_cell(ai_layer, index);
            let score = OptimalAi::minimax(board, false, ai_layer);
            board.clear_cell(index);

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        board.set_cell(ai_layer, best_index);
    }

    pub fn minimax(board: &mut Board, maximising: bool, ai_layer: &Layer) -> i32 {
        let state = board.get_state();
        let opp_layer = OptimalAi::opposite(ai_layer);

        match state {
            State::Win(layer) => {
                if layer == *ai_layer { return 1; }
                else { return -1; }
            },
            State::Draw => return 0,
            State::Unfinished => {},
        };

        let mut final_score = if maximising { -999 } else { 999 };

        for index in board.get_free_cells() {
            board.set_cell(if maximising {ai_layer} else {opp_layer}, index);
            let score = OptimalAi::minimax(board, !maximising, ai_layer);
            board.clear_cell(index);

            if maximising && score > final_score || !maximising && score < final_score {
                final_score = score;
            }
        }

        final_score
    }
}
