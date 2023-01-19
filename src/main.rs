use iced::futures::future::OrElse;
use iced::widget::{button, column, container, image, row, text, Space};
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
    Payouts,
}
struct VideoPoker {
    action: Actions,
    deck: Deck,
    bet: i8,
    last_bet: i8,
    credits: i32,
    hand: Vec<Card>,
}

#[derive(Clone, Debug)]
enum Message {
    CardHoldToggle(i32),
    BetMax,
    BetOne,
    Deal,
    GameStart,
    Payouts,
}

impl Application for VideoPoker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let deck = Deck::new();
        let action = Actions::Betting;
        let bet = 0;
        let last_bet = 5;
        let credits = 200;
        let hand = vec![
            Card::new(CardValue::Ace, Suits::Club),
            Card::new(CardValue::Two, Suits::Club),
            Card::new(CardValue::Three, Suits::Club),
            Card::new(CardValue::Four, Suits::Club),
            Card::new(CardValue::Five, Suits::Club),
        ];
        (
            Self {
                action,
                deck,
                bet,
                last_bet,
                credits,
                hand,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Video Poker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BetOne => {
                if self.bet <= 4 {
                    self.bet += 1;
                }
            }

            Message::CardHoldToggle(card) => {
                self.hand[card as usize].hold = !self.hand[card as usize].hold;
            }

            Message::BetMax => {
                self.bet = 5;
                self.action = Actions::Holding;
            }

            Message::Deal => match self.action {
                Actions::Betting => {
                    self.action = Actions::Holding;
                }

                Actions::Holding => {
                    self.action = Actions::Betting;
                }
                _ => {}
            },

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut bet_one = button("Bet One");
        let mut bet_max = button("Bet Max");
        let mut payouts = button("Payouts");
        let mut deal = button("Deal").on_press(Message::Deal);

        let mut action_holding = false;
        match self.action {
            Actions::Holding => {
                action_holding = true;
            }

            Actions::Betting => {
                bet_one = bet_one.on_press(Message::BetOne);
                bet_max = bet_max.on_press(Message::BetMax);
                payouts = payouts.on_press(Message::Payouts);
            }
            _ => {}
        }

        let mut cards = row![];
        for x in 0..5 {
            let hh = if self.hand[x as usize].hold {
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
                image(&self.hand[x].image_location).width(Length::Units(100)),
                Space::new(Length::Units(10), Length::Units(10)),
                button,
            ]);

            cards = cards.push( Space::new(Length::Units(30), Length::Units(20)));
        }

        let row_actions = row![bet_one, bet_max, payouts, deal,];

        let content: Element<_> = column![
            text("Video Poker").size(60),
            Space::new(Length::Units(20), Length::Units(20)),
            cards,
            Space::new(Length::Units(20), Length::Units(20)),
            row![
                text(format!("Bet - {}", self.bet)).size(20),
                Space::new(Length::Units(20), Length::Units(20)),
                text(format!("Credits - {}", self.credits)).size(20)
            ],
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
