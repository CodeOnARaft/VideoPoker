use iced::widget::{button, column, container, image, row, text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

mod card;
mod deck;

use crate::card::*;
use crate::deck::*;

fn main() -> iced::Result {
    VideoPoker::run(Settings::default())
}

enum Actions {
    Holding,
    Betting,
    HandOver,
    Payout,
}
struct VideoPoker {
    action: Actions,
    deck: Deck,
    bet: i8,
    last_bet:i8,
    credits:i32,
}

#[derive(Clone, Debug)]
enum Message {
    CardHoldToggle(i32),
    MaxBet,
    BetIncrease,
    BetDecrease,
    Deal,
    GameStart,
}

impl Application for VideoPoker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let deck = Deck::new();
        let action = Actions::Holding;
        let bet = 0;
        let last_bet = 5;
        let credits = 200;

        (Self { action, deck,bet,last_bet,credits }, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Video Poker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let hold_button_1 = button("Hold");
        let hold_button_2 = button("Hold");
        let hold_button_3 = button("Hold");
        let hold_button_4 = button("Hold");
        let hold_button_5 = button("Hold");

        let bet_one = button("Bet One");
        let bet_max = button("Bet Max");
        let payouts = button("Payouts");
        let deal = button("Deal");        

        let cards = row![
            image(&self.deck.cards[0].image_location).width(Length::Units(100)),
            image(&self.deck.cards[1].image_location).width(Length::Units(100)),
            image(&self.deck.cards[2].image_location).width(Length::Units(100)),
            image(&self.deck.cards[3].image_location).width(Length::Units(100)),
            image(&self.deck.cards[4].image_location).width(Length::Units(100)),
        ];



        match self.action {
            _ => {}
        }

        let row_hold = row![
            hold_button_1,
            hold_button_2,
            hold_button_3,
            hold_button_4,
            hold_button_5,            
        ];

        let row_actions = row![
              bet_one ,
              bet_max,
              payouts ,
              deal,
        ];

        let content: Element<_> = column![
            text("Video Poker").size(60),
            cards,
            row_hold,
            row![text("Bet - 0").size(20), text(format!("Credits - {}",self.credits)).size(20)],
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

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
