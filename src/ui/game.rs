use iced::widget::{button, column, container, image, row, text, Space};
use iced::{Element, Length};

use crate::{Message, Actions, VideoPoker};

pub struct Game {}

impl Game {
    pub fn view<'Message>(app:&VideoPoker) -> Element<'Message, Message> {
       
        let mut bet_one = button("Bet One");
        let mut bet_max = button("Bet Max");
        let mut payouts = button("Payouts");
        let deal = button("Deal").on_press(Message::Deal);

        let mut action_holding = false;
        match app.action {
            Actions::Holding => {
                action_holding = true;
            }

            Actions::Betting => {
                bet_one = bet_one.on_press(Message::BetOne);
                bet_max = bet_max.on_press(Message::BetMax);
                payouts = payouts.on_press(Message::Payouts);
            }
            
        }

        let mut cards = row![];
        for x in 0..5 {
            let hh = if app.hand[x as usize].hold {
                text("Holding").size(20)
            } else {
                text("       ").size(20)
            };

            let button = if action_holding {
                button("Hold").on_press(Message::CardHoldToggle(x as i32))
            } else {
                button("Hold")
            };
            cards = cards.push(column![
                hh,
                Space::new(Length::Units(10), Length::Units(10)),
                image(&app.hand[x].image_location).width(Length::Units(100)),
                Space::new(Length::Units(10), Length::Units(10)),
                button,
            ]);

            cards = cards.push(Space::new(Length::Units(30), Length::Units(20)));
        }

        let row_actions = row![
            bet_one,
            Space::new(Length::Units(20), Length::Units(20)),
            bet_max,
            Space::new(Length::Units(20), Length::Units(20)),
            payouts,
            Space::new(Length::Units(20), Length::Units(20)),
            deal,
        ];

        let content: Element<_> = column![
            text("Video Poker").size(60),
            Space::new(Length::Units(10), Length::Units(10)),
            cards,
            Space::new(Length::Units(10), Length::Units(10)),
            row![
                text(format!("Bet - {}", app.bet)).size(20),
                Space::new(Length::Units(20), Length::Units(20)),
                text(format!("Credits - {}", app.credits)).size(20)
            ],
            Space::new(Length::Units(10), Length::Units(10)),
            row_actions
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
