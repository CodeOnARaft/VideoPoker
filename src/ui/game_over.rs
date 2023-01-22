use iced::widget::{button, column, container, text, Space};
use iced::{Element, Length};

use crate::Message;

pub struct GameOver {}

impl GameOver {
    pub fn view<'Message>() -> Element<'Message, Message> {
        let row_message = column![
            text("You have run out of credits!").size(50).style(iced::Color::from_rgb(255.0f32, 0.0f32, 0.0f32)),
            Space::new(Length::Units(10), Length::Units(10)),
            text("Press the button to restart.").size(20),
        ];

        let content: Element<_> = column![
            text("Video Poker").size(60),
            Space::new(Length::Units(10), Length::Units(10)),
            row_message,
            Space::new(Length::Units(10), Length::Units(10)),
            button("Restart").on_press(Message::Restart),
        ]
        .into();

        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
