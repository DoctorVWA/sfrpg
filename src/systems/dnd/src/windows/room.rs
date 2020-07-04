use druid::piet::Color;
use druid::im::Vector;
use druid::lens::LensExt;
use druid::widget::{Align, Button, Flex, Label, SizedBox, TextBox, List, Scroll};
use druid::{Data, Lens, Widget, WidgetExt};


#[derive(Clone, Data, Lens)]
pub struct Room {
    pub url: String,
    pub name: String,
    pub player: Player,
    pub master: Option<Master>,
    pub players: Vector<Player>,
    pub messages: Vector<String>,
    pub characters: Vector<String>,
}

impl Default for Room {
    fn default() -> Room {
        Room {
            master: None,
            url: String::from(""),
            player: Player::default(),
            players: Vector::new(),
            messages: Vector::new(),
            characters: Vector::new(),
        }
    }
}

impl Room {
    pub fn builder() -> impl Widget<Room> {
        let chat_box = SizedBox::new(Room::chat_box())
            .border(Color::WHITE, 1.0)
            .fix_width(300.0)
            .fix_height(600.0);
        let action_box = SizedBox::new(Room::action_box())
            .border(Color::WHITE, 1.0)
            .fix_width(300.0)
            .fix_height(600.0);

        let layout = Flex::row()
            .with_child(chat_box)
            .with_spacer(40.0)
            .with_child(action_box);

        Align::centered(layout)
    }
    fn chat_box() -> impl Widget<Room> {
        Flex::column()
    }
    fn action_box() -> impl Widget<Room> {
        Flex::column()
    }
}

