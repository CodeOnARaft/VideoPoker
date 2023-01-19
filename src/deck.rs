use crate::card::*;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        cards.append(&mut Deck::get_suit(Suits::Club));
        cards.append(&mut Deck::get_suit(Suits::Diamond));
        cards.append(&mut Deck::get_suit(Suits::Heart));
        cards.append(&mut Deck::get_suit(Suits::Spade));       

        Self { cards }
    }

    fn get_suit(suit:Suits)-> Vec<Card>{
        vec![
            Card::new(CardValue::Two,suit.clone()),
            Card::new(CardValue::Three,suit.clone()),
            Card::new(CardValue::Four,suit.clone()),
            Card::new(CardValue::Five,suit.clone()),
            Card::new(CardValue::Six,suit.clone()),
            Card::new(CardValue::Seven,suit.clone()),
            Card::new(CardValue::Eight,suit.clone()),
            Card::new(CardValue::Nine,suit.clone()),
            Card::new(CardValue::Jack,suit.clone()),
            Card::new(CardValue::Queen,suit.clone()),
            Card::new(CardValue::King,suit.clone()),
            Card::new(CardValue::Ace,suit.clone()),
        ]
    }

    pub fn shuffle(&self) {

    }
}
