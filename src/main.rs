
#[macro_use]
mod units;
mod board;

use units::*;
use board::*;



custom_unit!(Cat);

impl Cat {

    fn new(level: u8) -> Self {
        let mut base_cat = Cat::empty();
        let base_unit: &mut UnitBase = base_cat.unit_mut();

        base_unit.health = (100*level).into();
        base_unit.damage = (50*level).into();

        base_unit.range = 1;
        base_unit.can_move = true;

        base_cat
    }
}

impl UnitActions for Cat {

    fn attack_phase(&mut self, board: &mut Board) {

    }
}



fn main() {
    let cat = Cat::new(1);
    let cell = make_cell(cat);

    if let Some(ref unit) = cell {
        println!("Unit health: {}", unit.borrow().unit().health);
    } else {
        println!("No unit available.");
    }

    let mut board = Board::new();

    if board.unit_at((0, 0)).is_none() {
        println!("First space is empty.");
    }

    board.grid[0][0] = cell;
    
    if let Some(ref mut unit) = board.unit_at_mut((0, 0)) {
        println!("First space filled!");

        unit.unit_mut().health = 20;

        println!("Unit health after update: {}", unit.unit().health);
    };
}
