#[derive(PartialEq, Debug)]
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

pub struct Board {
    cross_layer: u32,
    nought_layer: u32,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cross_layer: 0,
            nought_layer: 0,
        }
    }

    pub fn print_board(&self) {
        for i in 0..3 {
            println!("+---+---+---+");
            print!("|");
            for j in 0..3 {
                let index = i * 3 + j;

                if self.cross_layer & 1 << index != 0 {
                    print!(" X ");
                } else if self.nought_layer & 1 << index != 0 {
                    print!(" O ");
                } else {
                    print!(" {} ", index);
                }
                print!("|");
            }
            print!("\n");
        }
        println!("+---+---+---+");
    }

    pub fn set_cell(&mut self, mark: &Mark, index: u32) {
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
        };

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

        for mask in win_masks.iter() {
            if self.cross_layer & mask == *mask { return State::Win(Mark::Cross); }
            if self.nought_layer & mask == *mask { return State::Win(Mark::Nought); }
        }

        if self.cross_layer | self.nought_layer == 0b111_111_111 { return State::Draw; }

        State::Unfinished
    }
}
