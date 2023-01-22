use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};

use crate::Message;

pub struct Payouts {}

impl Payouts {
    pub fn view<'Message>() -> Element<'Message, Message> {
        let column1 = column![
            text("ROYAL FLUSH").size(20),
            text("STRAIGHT FLUSH").size(20),
            text("FOUR OF A KIND").size(20),
            text("FULL HOUSE").size(20),
            text("FLUSH").size(20),
            text("STRAIGHT").size(20),
            text("THREE OF A KIND").size(20),
            text("TWO PAIR").size(20),
            text("JACKS OR BETTER").size(20),
        ];
        let column2 = column![
            text("250").size(20),
            text("50").size(20),
            text("25").size(20),
            text("9").size(20),
            text("6").size(20),
            text("4").size(20),
            text("3").size(20),
            text("2").size(20),
            text("1").size(20),
        ];

        let column3 = column![
            text("500").size(20),
            text("100").size(20),
            text("50").size(20),
            text("18").size(20),
            text("12").size(20),
            text("8").size(20),
            text("6").size(20),
            text("4").size(20),
            text("2").size(20),
        ];

        let column4 = column![
            text("750").size(20),
            text("150").size(20),
            text("75").size(20),
            text("27").size(20),
            text("18").size(20),
            text("12").size(20),
            text("9").size(20),
            text("6").size(20),
            text("3").size(20),
        ];


        let column5 = column![
            text("1000").size(20),
            text("200").size(20),
            text("100").size(20),
            text("36").size(20),
            text("24").size(20),
            text("16").size(20),
            text("12").size(20),
            text("8").size(20),
            text("4").size(20),
        ];

        let column6 = column![
            text("4000").size(20),
            text("250").size(20),
            text("125").size(20),
            text("45").size(20),
            text("30").size(20),
            text("20").size(20),
            text("15").size(20),
            text("10").size(20),
            text("5").size(20),
        ];


        let row_winnings = row![
            column1,
            Space::new(Length::Units(10), Length::Units(10)),
            column2,
            Space::new(Length::Units(10), Length::Units(10)),
            column3,
            Space::new(Length::Units(10), Length::Units(10)),
            column4,
            Space::new(Length::Units(10), Length::Units(10)),
            column5,
            Space::new(Length::Units(10), Length::Units(10)),
            column6,
            
        ];

        let content: Element<_> = column![
            text("Video Poker").size(60),
            Space::new(Length::Units(10), Length::Units(10)),
            row_winnings,
            Space::new(Length::Units(10), Length::Units(10)),
            button("Close").on_press(Message::Payouts),
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
