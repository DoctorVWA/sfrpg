use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{Data, Lens, Widget, WidgetExt};

#[derive(Clone, Data, Default, Lens)]
pub struct Login {
    pub ip: String,
    pub port: String,
    pub username: String,
    pub error: String
}

impl Login {
    pub fn builder() -> impl Widget<Login> {
        let err_lbl = Label::dynamic(|data, _| format!("{}", data).to_string())
            .lens(Login::error);
        let lbl_ip = Label::new("IP Address:");
        let txt_ip = TextBox::new()
            .with_placeholder("127.0.0.1")
            .fix_width(150.0)
            .fix_height(30.0)
            .lens(Login::ip);

        let lbl_port = Label::new("Port:");
        let txt_port = TextBox::new()
            .with_placeholder("8080")
            .fix_width(150.0)
            .fix_height(30.0)
            .lens(Login::port);

        let lbl_username = Label::new("Username:");
        let txt_username = TextBox::new()
            .with_placeholder("Clébinho")
            .fix_width(150.0)
            .fix_height(30.0)
            .lens(Login::username);

        let btn_connect = Button::new("Connect" /*TODO localized string*/)
            .on_click(|_ctx, window: &mut Login, _env| {
                if &*window.ip == "" {
                    window.error = String::from("Ip não especificado");
                } else if &*window.port == "" {
                    window.error = String::from("Porta não especificada");
                } else if &*window.username == "" {
                    window.error = String::from("Usuario não especificado");
                } else {
                    window.error = String::from("");
                    println!("{:?} {:?} {:?}", window.ip, window.port, window.username);
                }
                /*TODO connection*/
            });

        let flex = Flex::column()
            .with_child(err_lbl)
            .with_spacer(15.0)
            .with_child(lbl_ip)
            .with_child(txt_ip)
            .with_spacer(4.0)
            .with_child(lbl_port)
            .with_child(txt_port)
            .with_spacer(4.0)
            .with_child(lbl_username)
            .with_child(txt_username)
            .with_spacer(8.0)
            .with_child(btn_connect);

        Align::centered(flex)
    }
}
