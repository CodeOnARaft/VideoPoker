use iced::widget::{button, column, container, text, row,image};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

fn main() -> iced::Result {
    VideoPoker::run(Settings::default())
}

enum VideoPoker {
    Holding,
    Betting,
    HandOver,
    Payout,
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
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::Betting, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Video Poker")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
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
                    image(format!("{}/images/2_of_clubs.png", env!("CARGO_MANIFEST_DIR"))) .width(Length::Units(100)),
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
}
