
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;
use std::rc::Rc;
use std::rc::Weak;
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

    pub fn new() -> Board<'a> {
        Board {
            grid: Default::default(),
            units: vec![]
        }
    }

    pub fn place_piece(&mut self, cell: GridCell<'a>, pos: (u8, u8)) {
        let (x, y) = pos;
        let board_space = &mut self.grid[x as usize][y as usize];
        
        // Evict current tennant
        let current_unit: &mut GridCell<'a> = board_space;
        if let Some(ref unit) = current_unit {
            self.units.retain(|ref potential_unit| {
                match potential_unit.upgrade() {
                    Some(active_unit) =>
                        active_unit.borrow().unit() != unit.borrow().unit(),
                    None => false
                }
            });

            let mut borrow = unit.borrow_mut();
            let unit_ref = borrow.unit_mut();
            unit_ref.pos = None;
        }

        // Place new tennant
        match cell {
            Some(ref unit) => {
                self.units.push(Rc::downgrade(unit));

                unit.borrow_mut().unit_mut().pos = Some(pos);
                *board_space = cell;
            },
            None => *board_space = None
        }
    }

    pub fn unit_at(&self, pos: (u8, u8)) -> Option<Ref<Unit + 'a>> {
        let grid_pos = &self.grid[pos.0 as usize][pos.1 as usize];

        match grid_pos {
            Some(ref rc_unit) => Some(rc_unit.borrow()),
            None => None
        }
    }

    pub fn unit_at_mut(&self, pos: (u8, u8)) -> Option<RefMut<Unit + 'a>> {
        let grid_pos = &self.grid[pos.0 as usize][pos.1 as usize];

        match grid_pos {
            Some(ref rc_unit) => Some(rc_unit.borrow_mut()),
            None => None
        }
    }
}
