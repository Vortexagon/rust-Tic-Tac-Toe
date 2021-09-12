use crate::board::{Board, Mark, State};
use rand::{self, Rng};
use std::cmp;
use std::collections::HashMap;
use std::time::Instant;

pub struct OptimalAi;

impl OptimalAi {
    fn opposite(mark: Mark) -> Mark {
        if mark == Mark::Cross {Mark::Nought} else {Mark::Cross}
    }

    pub fn make_move(board: &mut Board, ai_mark: Mark) {
        let now = Instant::now();

        let mut best_score = -999;
        let mut best_index = 0;

        let mut cache = ZobristCache::new(board);

        for index in board.get_free_cells() {
            let zobrist_key = cache.keys[index as usize][if ai_mark == Mark::Nought { 0 } else { 1 }];

            board.set_cell(ai_mark, index);
            cache.state ^= zobrist_key;

            let score = OptimalAi::minimax(board, -999, 999, false, ai_mark, &mut cache);

            board.clear_cell(index);
            cache.state ^= zobrist_key;

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        board.set_cell(ai_mark, best_index);
        println!(
            "Took {} milliseconds to choose move!",
            now.elapsed().as_millis()
        );
    }

    fn minimax(
        board: &mut Board,
        mut alpha: i32,
        mut beta: i32,
        maximising: bool,
        ai_mark: Mark,
        cache: &mut ZobristCache,
    ) -> i32 {
        let state = board.get_state();
        let curr_mark = if maximising { ai_mark } else { OptimalAi::opposite(ai_mark) } ;

        match state {
            State::Win(win_mark) => return if win_mark == ai_mark { 1 } else { -1 },
            State::Draw => return 0,
            State::Unfinished => (),
        };

        if let Some(val) = cache.data.get(&cache.state) {
            return *val;
        }

        let mut final_score = if maximising { -999 } else { 999 };

        for index in board.get_free_cells() {

            let zobrist_key = cache.keys[index as usize][if curr_mark == Mark::Nought { 0 } else { 1 }];

            board.set_cell(curr_mark, index);
            cache.state ^= zobrist_key;

            let score = OptimalAi::minimax(board, alpha, beta, !maximising, ai_mark, cache);

            board.clear_cell(index);
            cache.state ^= zobrist_key;

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

        cache.data.insert(cache.state, final_score);
        final_score
    }
}

struct ZobristCache {
    state: u64,
    keys: [[u64; 2]; 9],
    data: HashMap<u64, i32>,
}

impl ZobristCache {
    fn new(board: &Board) -> Self {
        let mut rng = rand::thread_rng();
        let mut vals = [[0u64; 2]; 9];
        let mut initial_state: u64 = 0;

        (0..9).for_each(|cell| {
            (0..2).for_each(|col| vals[cell][col] = rng.gen())
        });

        for i in 0..9 {
            if board.nought_layer & (1 << i) != 0 {
                initial_state ^= vals[i][0];
            } else if board.cross_layer & (1 << i) != 0 {
                initial_state ^= vals[i][1];
            }
        }

        Self {
            state: initial_state,
            keys: vals,
            data: HashMap::new(),
        }
    }
}
