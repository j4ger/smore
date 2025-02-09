use iced::{
    widget::{column, container},
    Element,
};

#[derive(Debug, Clone)]
pub struct Login {
    username: String,
    password: String,
}

impl Default for Login {
    fn default() -> Self {
        Login {
            username: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Login {
    pub fn view(&self) -> Element<Message> {
        column![container("Hi")].into()
    }

    pub fn update(&mut self, event: Message) {}
}
