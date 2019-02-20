
use crate::units::Unit;


pub type GridCell<'a> = Option<Box<Unit + 'a>>;

pub struct Board<'a> {
    pub grid: [[GridCell<'a>; 8]; 8],
}

pub fn make_cell<'a>(unit: impl Unit + 'a) -> GridCell<'a> {
    Some(Box::from(unit))
}