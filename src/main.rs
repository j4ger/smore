mod views;

struct Smore {
    screen: Screen,
}

impl Default for Smore {
    fn default() -> Self {
        Smore {
            screen: Screen::Login(Default::default()),
        }
    }
}

#[derive(Debug, Clone)]
enum Screen {
    Login(views::Login),
}

#[derive(Debug, Clone)]
enum Message {
    Login(views::LoginMessage),
}

impl Smore {
    fn title(&self) -> String {
        match self.screen {
            Screen::Login(_) => "Smore - Login".to_string(),
        }
    }

    fn update(&mut self, event: Message) {
        match &self.screen {
            Screen::Login(inner) => {}
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Login(inner) => inner.view().map(Message::Login),
        }
    }
}

fn main() -> iced::Result {
    iced::application(Smore::title, Smore::update, Smore::view).run()
}
