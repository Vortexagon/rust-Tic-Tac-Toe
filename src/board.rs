use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Mark {
    Cross,
    Nought,
    Empty,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Unfinished,
    Draw,
    Win(Mark),
}

#[derive(Default)]
pub struct Board {
    pub(crate) cross_layer: u32,
    pub(crate) nought_layer: u32,
}

impl Board {
    pub fn set_cell(&mut self, mark: Mark, index: u32) {
        match mark {
            Mark::Cross => self.cross_layer |= 1 << index,
            Mark::Nought => self.nought_layer |= 1 << index,
            Mark::Empty => self.clear_cell(index),
        }
    }

    pub fn clear_cell(&mut self, index: u32) {
        self.cross_layer &= !(1 << index);
        self.nought_layer &= !(1 << index);
    }

    pub fn get_free_cells(&self) -> Vec<u32> {
        let mut free_cells = Vec::new();
        let free_board = !(self.cross_layer | self.nought_layer);

        for i in 0..9 {
            if free_board & 1 << i != 0 {
                free_cells.push(i);
            }
        }

        free_cells
    }

    pub fn get_state(&self) -> State {
        let win_masks = [
            0b111_000_000,
            0b000_111_000,
            0b000_000_111,
            0b100_100_100,
            0b010_010_010,
            0b001_001_001,
            0b100_010_001,
            0b001_010_100,
        ];

        for mask in win_masks {
            if self.cross_layer & mask == mask {
                return State::Win(Mark::Cross);
            }
            if self.nought_layer & mask == mask {
                return State::Win(Mark::Nought);
            }
        }

        if self.cross_layer | self.nought_layer == 0b111_111_111 {
            return State::Draw;
        }

        State::Unfinished
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut result_str = String::with_capacity(169);

        for i in 0..3 {
            result_str.push_str("+---+---+---+\n|");
            for j in 0..3 {
                let index = i * 3 + j;

                if self.cross_layer & 1 << index != 0 {
                    result_str.push_str(" X ");
                } else if self.nought_layer & 1 << index != 0 {
                    result_str.push_str(" O ");
                } else {
                    result_str.push_str(&*format!(" {} ", index));
                }
                result_str.push_str("|");
            }
            result_str.push_str("\n");
        }
        result_str.push_str("+---+---+---+\n");

        write!(f, "{}", result_str)
    }
}
