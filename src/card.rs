#[derive(Clone)]
pub enum Suits {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
}

pub struct Card {
    pub suit: Suits,
    pub value: CardValue,
    pub hold: bool,
    pub image_location: String
}

impl Card {
    pub fn new(value: CardValue, suit: Suits) -> Card {
        let hold = false;
        let image_location = Card::get_image_location(&value,&suit);
        Self {
            suit,
            value,
            hold,         
            image_location,  
        }
    }

    fn get_image_location(value: &CardValue, suit: &Suits) -> String{
        let val_str = match value {
            CardValue::Two=> {"2"}
            CardValue::Three=> {"3"}
            CardValue::Four=> {"4"}
            CardValue::Five=> {"5"}
            CardValue::Six=> {"6"}
            CardValue::Seven=> {"7"}
            CardValue::Eight=> {"8"}
            CardValue::Nine=> {"9"}
            CardValue::Jack=> {"jack"}
            CardValue::Queen=> {"queen"}
            CardValue::King=> {"king"}
            CardValue::Ace=> {"ace"}
        };

        let suit_str = match suit {
            Suits::Club=> {"clubs"}
            Suits::Diamond=> {"diamonds"}
            Suits::Heart=> {"hearts"}
            Suits::Spade=> {"spades"}
        };

        format!("{}/images/{}_of_{}.png", env!("CARGO_MANIFEST_DIR"),val_str,suit_str)
    } 

}
