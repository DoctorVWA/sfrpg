use druid::im::Vector;
use druid::{Data, Lens, Widget, WidgetExt};

#[derive(Clone, Data, Lens)]
pub struct Race {
    
}

#[derive(Clone, Data, Lens)]
pub struct Character {
    pub name: String,
    pub race: Race
}

#[derive(Clone, Data, Lens)]
pub struct Player {
    pub name: String,
    pub characters: Vector<Character>
}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from(""),
            characters: Vector::new()
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Master {
    pub name: String,
    pub npcs: Vector<Character>
}

impl Default for Master {
    fn default() -> Master {
        Master {
            name: String::from(""),
            npcs: Vector::new()
        }
    }
}
