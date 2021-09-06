#[derive(PartialEq, Debug)]
pub enum Layer {
    X,
    O,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Unfinished,
    Draw,
    Win(Layer),
}

pub struct Board {
    x_layer: u32,
    o_layer: u32,
}


impl Board {
    pub fn new() -> Board {
        Board {
            x_layer: 0,
            o_layer: 0,
        }
    }


    pub fn print_board(&self) {
        for i in 0..3 {
            println!("\n+---+---+---+");
            print!("|");
            for j in 0..3 {
                let index = i * 3 + j;

                if self.x_layer & 1 << index != 0 {
                    print!(" X |");
                } else if self.o_layer & 1 << index != 0 {
                    print!(" O |");
                } else {
                    print!(" {} |", index);
                }
            }
        }
        println!("\n+---+---+---+");
    }


    pub fn set_cell(&mut self, layer: &Layer, index: u32) {
        match layer {
            &Layer::X => self.x_layer |= 1 << index,
            &Layer::O => self.o_layer |= 1 << index,
        }
    }

    pub fn clear_cell(&mut self, index: u32) {
        self.x_layer &= !(1 << index);
        self.o_layer &= !(1 << index);
    }

    pub fn get_free_cells(&self) -> Vec<u32> {
        let mut free_cells = Vec::new();
        let free_board = !(self.x_layer | self.o_layer);

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
            if self.x_layer & mask == *mask { return State::Win(Layer::X); }
            if self.o_layer & mask == *mask { return State::Win(Layer::O); }
        }

        if self.x_layer | self.o_layer == 0b111_111_111 { return State::Draw; }

        State::Unfinished
    }
}
