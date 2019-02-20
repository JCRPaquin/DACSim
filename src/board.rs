
use std::rc::*;
use std::cell::RefCell;
use crate::units::Unit;


pub type GridCell<'a> = Option<Rc<RefCell<Unit + 'a>>>;

pub struct Board<'a> {
    pub grid: [[GridCell<'a>; 8]; 8],
    pub units: Vec<Weak<RefCell<Unit + 'a>>>
}

pub fn make_cell<'a>(unit: impl Unit + 'a) -> GridCell<'a> {
    Some(Rc::new(RefCell::new(unit)))
}

impl<'a> Board<'a> {

    fn place_piece(&mut self, cell: GridCell<'a>, x: u8, y: u8) {
        let board_space = &mut self.grid[x as usize][y as usize];
        
        // Evict current tennant
        let current_unit: GridCell<'a> = *board_space;
        if let Some(ref unit) = current_unit {
            self.units.retain(|&potential_unit| {
                match potential_unit.upgrade() {
                    Some(active_unit) =>
                        active_unit.borrow().unit() != unit.borrow().unit(),
                    None => false
                }
            });

            let unit_ref = unit.borrow_mut().unit_mut();
            unit_ref.pos = None;
        }

        // Place new tennant
        match cell {
            Some(ref unit) => {
                *board_space = cell;
                self.units.push(Rc::downgrade(unit));
            },
            None => *board_space = None
        }
    }
}
