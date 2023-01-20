use iced::widget::{button, column, container, image, row, text, Space};
use iced::{executor, window, Application, Command, Element, Length, Settings, Theme};

mod card;
mod deck;

use crate::card::*;
use crate::deck::*;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (700, 450),
            resizable: false,
            decorations: true,
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    };
    VideoPoker::run(settings)
}

enum Actions {
    Holding,
    Betting,
}
struct VideoPoker {
    action: Actions,
    deck: Deck,
    bet: i32,
    last_bet: i32,
    credits: i32,
    hand: Vec<Card>,
    show_payouts: bool,
}

#[derive(Clone, Debug)]
enum Message {
    CardHoldToggle(i32),
    BetMax,
    BetOne,
    Deal,
    Payouts,
}

impl Application for VideoPoker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut deck = Deck::new();
        deck.shuffle();

        let action = Actions::Betting;
        let bet = 0;
        let last_bet = 5;
        let credits = 200;
        let hand = vec![
            Card::back(),
            Card::back(),
            Card::back(),
            Card::back(),
            Card::back(),
        ];
        let show_payouts = false;
        (
            Self {
                action,
                deck,
                bet,
                last_bet,
                credits,
                hand,
                show_payouts,
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

                self.deal();
            }

            Message::Deal => match self.action {
                Actions::Betting => {
                    if self.bet == 0 {
                        self.bet = self.last_bet;
                    }
                    self.deal();
                }

                Actions::Holding => {
                    for x in 0..5 {
                        if !self.hand[x].hold {
                            self.hand[x] = self.deck.next();
                        }
                    }
                    self.check_win();

                    self.deck.shuffle();
                    self.bet = 0;
                    self.action = Actions::Betting;
                }
                
            },

            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut bet_one = button("Bet One");
        let mut bet_max = button("Bet Max");
        let mut payouts = button("Payouts");
        let deal = button("Deal").on_press(Message::Deal);

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
                text(format!("Bet - {}", self.bet)).size(20),
                Space::new(Length::Units(20), Length::Units(20)),
                text(format!("Credits - {}", self.credits)).size(20)
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

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl VideoPoker {
    fn check_win(&mut self) {
        let flush = self.check_flush();

        let values = self.get_card_values();
        let (straight, ace_high) = VideoPoker::check_straight(&values);
        let royal = straight & flush & ace_high;
        let (fourkind, threekind, pair,two_pair, jacks_or_better) = VideoPoker::check_matching(&values);
        let fullhouse = threekind & pair;

        self.payout(
            royal,
            flush,
            straight,
            fullhouse,
            fourkind,
            threekind,
            two_pair,
            jacks_or_better,
        );
    }

    fn check_matching(values: &Vec<i16>) -> (bool, bool, bool, bool,bool) {
        let mut fourkind = false;
        let mut threekind = false;
        let mut pair = false;
        let mut jacks_or_better = false;
        let mut two_pair = false;

        for x in 0..14 {
            match values[x] {
                2 => {
                    two_pair = pair;
                    pair = true;
                    jacks_or_better = jacks_or_better | (x >= 10);
                }
                3 => threekind = true,
                4 => fourkind = true,
                _ => {}
            }
        }

        (fourkind, threekind, pair,two_pair, jacks_or_better)
    }

    fn check_straight(values: &Vec<i16>) -> (bool, bool) {
        let mut index = 0;
        for x in 2..10 {
            if values[x] == 1 {
                index = x;
                break;
            }
        }

        let straight = (values[index + 2] == 1)
            & (values[index + 3] == 1)
            & (values[index + 4] == 1)
            & (values[index + 5] == 1);

        let mut ace_high = false;

        if straight {
            ace_high = values[13] == 1;
        }

        (straight, ace_high)
    }

    fn check_flush(&self) -> bool {
        (self.hand[0].suit == self.hand[1].suit)
            & (self.hand[1].suit == self.hand[2].suit)
            & (self.hand[2].suit == self.hand[3].suit)
            & (self.hand[3].suit == self.hand[4].suit)
    }

    fn get_card_values(&self) -> Vec<i16> {
        let mut values = vec![0; 14];

        for x in 0..5 {
            match self.hand[x].value {
                CardValue::Number(val) => values[val as usize] += 1,
                CardValue::Jack => values[10] += 1,
                CardValue::Queen => values[11] += 1,
                CardValue::King => values[12] += 1,
                CardValue::Ace => values[13] += 1,
                _ => {}
            }
        }

        values
    }

    fn deal(&mut self) {

        self.hand = vec![
            self.deck.next(),
            self.deck.next(),
            self.deck.next(),
            self.deck.next(),
            self.deck.next(),
        ];

        self.last_bet = self.bet;
        self.credits -= self.bet;
        self.action = Actions::Holding;
    }

    fn payout(
        &mut self,
        royal: bool,
        flush: bool,
        straight: bool,
        fullhouse: bool,
        fourkind: bool,
        threekind: bool,
        two_pair:bool,
        jacks_or_better: bool,
    ) {
        let payout_royal = vec![250, 500, 750, 1000];
        let payout_straight_flush = vec![50, 100, 150, 200, 250];
        let payout_fourkind = vec![25, 50, 75, 100];

        if self.bet < 1 {
            self.bet = 1
        };
        if self.bet > 5 {
            self.bet = 5
        };

        if royal {
            self.credits += payout_royal[(self.bet - 1) as usize];
        } else if flush && straight {
            self.credits += payout_straight_flush[(self.bet - 1) as usize];
        } else if fourkind {
            self.credits += payout_fourkind[(self.bet - 1) as usize];
        } else if fullhouse {
            self.credits += 9 * self.bet ;
        } else if flush {
            self.credits += 6 * self.bet;
        } else if straight {
            self.credits += 4 * self.bet;
        } else if threekind {
            self.credits += 3 * self.bet;
        } else if two_pair {
            self.credits += 2 * self.bet;
        }
        if jacks_or_better {
            self.credits += self.bet;
        }
    }
}
