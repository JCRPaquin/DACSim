
use crate::board::Board;


// Contains all the fields used by other unit types
#[derive(PartialEq, Eq, Hash)]
pub struct UnitBase {
    pub health: u16,
    pub damage: u16,

    pub range: u8,
    pub can_move: bool,

    pub delay: u8,
    pub invulnerable: bool
}

// Used later in the unit type macro
impl Default for UnitBase {

    fn default() -> UnitBase {
        UnitBase {
            health: 0,
            damage: 0,

            range: 0,
            can_move: false,

            delay: 0,
            invulnerable: false
        }
    }
}

pub trait UnitInner {

    fn unit(&self) -> &UnitBase;

    fn unit_mut(&mut self) -> &mut UnitBase;

}


pub trait UnitActions {

    fn attack_phase(&mut self, board: &mut Board);

}

pub trait Unit : UnitInner + UnitActions {}
impl<T: UnitInner + UnitActions> Unit for T {}


#[macro_export]
macro_rules! custom_unit {
    ( $name:ident ) => {
        #[derive(PartialEq, Eq, Hash)]
        struct $name {
            unit: UnitBase
        }

        impl UnitInner for $name {

            fn unit(&self) -> &UnitBase {
                &self.unit
            }

            fn unit_mut(&mut self) -> &mut UnitBase {
                &mut self.unit
            }

        }

        impl $name {

            fn empty() -> Self {
                $name {
                    unit: Default::default()
                }
            }
        }
    };
}
