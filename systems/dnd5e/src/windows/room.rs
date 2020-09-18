use druid::piet::Color;
use druid::im::Vector;
use druid::widget::{Align, Button, Flex, Label, SizedBox, TextBox, List, Scroll, CrossAxisAlignment};
use druid::{Data, Lens, Widget, WidgetExt, UnitPoint};

use crate::core::{
    Player, Master
};


#[derive(Clone, Data, Lens)]
pub struct Message {
    pub content: String,
    pub author: String,
    pub date: String /*rust date thing*/
}

#[derive(Clone, Data, Lens)]
pub struct Room {
    pub url: String,
    pub name: String,
    pub player: Player,
    pub message: String,
    pub master: Option<Master>,
    pub players: Vector<Player>,
    pub messages: Vector<Message>,
    pub characters: Vector<String>,
}

impl Default for Room {
    fn default() -> Room {
        Room {
            master: None,
            url: String::from(""),
            name: String::from(""),
            message: String::from(""),
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
        let mut flex = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

        flex.add_flex_child(
            Scroll::new(List::new(|| {
                Label::new(|item: &Message, _env: &_| format!("{}: {}", item.author, item.content))
                    .align_vertical(UnitPoint::TOP_LEFT)
                    .padding(10.0)
                    .expand()
                    .height(30.0)
            }))
            .vertical()
            .lens(Room::messages),
            1.0
        );

        flex.add_flex_child(
            TextBox::new()
                .align_vertical(UnitPoint::BOTTOM_LEFT)
                .expand()
                .height(20.0)
                .lens(Room::message),
            1.0
        );

        flex.add_flex_child(
            Button::new("Send")
                .on_click(|_ctx, room: &mut Room, _env| {
                    room.messages.push_back(Message {
                        content: room.message.clone(),
                        author: "Joãozin".to_string(),
                        date: "23/09/2020 23:23".to_string()
                    });
                    room.message = "".into();
                }),
            1.0
        );

        flex
    }
    fn action_box() -> impl Widget<Room> {
        Flex::column()
    }
}

