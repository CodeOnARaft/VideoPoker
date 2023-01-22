use iced::{executor, window, Application, Command, Element, Settings, Theme};

mod card;
mod deck;
mod ui;

use crate::card::*;
use crate::deck::*;
use crate::ui::*;

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

pub enum Actions {
    Holding,
    Betting,
}
pub struct VideoPoker {
    action: Actions,
    deck: Deck,
    bet: i32,
    last_bet: i32,
    credits: i32,
    hand: Vec<Card>,
    show_payouts: bool,
    show_game_over :bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    CardHoldToggle(i32),
    BetMax,
    BetOne,
    Deal,
    Payouts,
    Restart,
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
        let show_game_over = false;
        (
            Self {
                action,
                deck,
                bet,
                last_bet,
                credits,
                hand,
                show_payouts,
                show_game_over,
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

            Message::Payouts =>{
                self.show_payouts = !self.show_payouts;
            }

            Message::Restart=>{
                self.credits = 200;
                self.show_game_over = false;
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
                    if self.credits == 0 {
                        self.show_game_over = true;
                    }
                }
                
            },

            
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {

        if self.show_payouts{
            return payouts::Payouts::view();
        }

        if self.show_game_over {
            return game_over::GameOver::view();
        }

        game::Game::view(self)
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

        for x in 0..15 {
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
        for x in 2..11 {
            if values[x] == 1 {
                index = x;
                break;
            }
        }

        let straight = (values[index + 1] == 1)
            & (values[index + 2] == 1)
            & (values[index + 3] == 1)
            & (values[index + 4] == 1);

        let mut ace_high = false;

        if straight {
            ace_high = values[14] == 1;
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
        let mut values = vec![0; 15];

        for x in 0..5 {
            match self.hand[x].value {
                CardValue::Number(val) => values[val as usize] += 1,
                CardValue::Jack => values[11] += 1,
                CardValue::Queen => values[12] += 1,
                CardValue::King => values[13] += 1,
                CardValue::Ace => values[14] += 1,
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
        let payout_royal = vec![250, 500, 750, 1000,4000];
        let payout_straight_flush = vec![50, 100, 150, 200, 250];
        let payout_fourkind = vec![25, 50, 75, 100,125];

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
