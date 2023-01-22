#[derive(Clone, PartialEq, Eq)]
pub enum Suits {
    Back,
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CardValue {
    Back,
    Number(i32),
    Jack,
    Queen,
    King,
    Ace,
}


#[derive(Clone)]
pub struct Card {
    pub suit: Suits,
    pub value: CardValue,
    pub hold: bool,
    pub image_location: String,
}

impl Card {
    pub fn new(value: CardValue, suit: Suits) -> Card {
        let hold = false;
        let image_location = Card::get_image_location(&value, &suit);
        Self {
            suit,
            value,
            hold,
            image_location,
        }
    }

    pub fn back() -> Card {
        let image_location = format!("{}/images/blue_back.png", env!("CARGO_MANIFEST_DIR"));
        let suit = Suits::Back;
        let value = CardValue::Back;
        let hold = false;
        Self {
            suit,
            value,
            hold,
            image_location,
        }
    }

    fn get_image_location(value: &CardValue, suit: &Suits) -> String {
        let temp:String;
        let val_str = match value {
            CardValue::Number(num) => {
                temp = num.to_string();
                temp.as_str()
            },            
            CardValue::Jack => "jack",
            CardValue::Queen => "queen",
            CardValue::King => "king",
            CardValue::Ace => "ace",
            CardValue::Back => "",
        };

        let suit_str = match suit {
            Suits::Club => "clubs",
            Suits::Diamond => "diamonds",
            Suits::Heart => "hearts",
            Suits::Spade => "spades",
            Suits::Back => "",
        };

        format!(
            "{}/images/{}_of_{}.png",
            env!("CARGO_MANIFEST_DIR"),
            val_str,
            suit_str
        )
    }
}
