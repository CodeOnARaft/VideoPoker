use iced::widget::{button, column, container, text, row,image};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

mod deck;
mod card;

use crate::deck::*;
use crate::card::*;


fn main() -> iced::Result {
    VideoPoker::run(Settings::default())
}

enum Actions{
    Holding,
    Betting,
    HandOver,
    Payout, 
}
struct VideoPoker {
   action:Actions,
    deck:Deck
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
        let deck = Deck::new() ;
        let action = Actions::Holding;
        (Self{            action,deck           
        }, iced::Command::none())
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
        let content: Element<_> = match self {
          
            _ => {
                let row_hold = row![
                    button("Hold").on_press(Message::CardHoldToggle(1)),
                    button("Hold").on_press(Message::CardHoldToggle(2)),
                    button("Hold").on_press(Message::CardHoldToggle(3)),
                    button("Hold").on_press(Message::CardHoldToggle(4)),
                    button("Hold").on_press(Message::CardHoldToggle(5)),
                ];

                let cards = row![
                    image(self.deck.cards[0].image_location.clone()) .width(Length::Units(100)),
                    image(self.deck.cards[1].image_location.clone()) .width(Length::Units(100)),
                    image(self.deck.cards[2].image_location.clone()) .width(Length::Units(100)),
                    image(self.deck.cards[3].image_location.clone()) .width(Length::Units(100)),
                    image(self.deck.cards[4].image_location.clone()) .width(Length::Units(100)),
                ];

                column![text("Video Poker").size(40), 
                text("Payouts").size(60),
                cards,
                row_hold,
                row![text("Bet - 0").size(40),text("Credits").size(40)],
                ].into()
            }
        };

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
