use crate::card::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
    last: i32,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        cards.append(&mut Deck::get_suit(Suits::Club));
        cards.append(&mut Deck::get_suit(Suits::Diamond));
        cards.append(&mut Deck::get_suit(Suits::Heart));
        cards.append(&mut Deck::get_suit(Suits::Spade));
        let last = 0;
        Self { cards, last }
    }

    fn get_suit(suit: Suits) -> Vec<Card> {
        vec![
            Card::new(CardValue::Two, suit.clone()),
            Card::new(CardValue::Three, suit.clone()),
            Card::new(CardValue::Four, suit.clone()),
            Card::new(CardValue::Five, suit.clone()),
            Card::new(CardValue::Six, suit.clone()),
            Card::new(CardValue::Seven, suit.clone()),
            Card::new(CardValue::Eight, suit.clone()),
            Card::new(CardValue::Nine, suit.clone()),
            Card::new(CardValue::Jack, suit.clone()),
            Card::new(CardValue::Queen, suit.clone()),
            Card::new(CardValue::King, suit.clone()),
            Card::new(CardValue::Ace, suit.clone()),
        ]
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
        self.last = 0;
    }

    pub fn next(&mut self) -> Card {
        if self.last >= 51 {
            self.shuffle();
        }
        let card = self.cards[self.last as usize].clone();
        self.last += 1;

        card
    }
}
