use serde::{Deserialize, Serialize};

use crate::model::card::{Deck, DiscardPile, Hand};

#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
    points: u16,
    deck: Deck,
    hand: Hand,
    discard_pile: DiscardPile,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let mut deck = Deck::shuffled();
        let mut hand = Hand::default();
        hand.draw_cards_from_deck(&mut deck, 5);
        Player {
            name: name.to_owned(),
            points: 100,
            deck,
            hand,
            discard_pile: DiscardPile::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn points(&self) -> u16 {
        self.points
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn deck_size(&self) -> usize {
        self.deck.cards().len()
    }

    pub fn hand_size(&self) -> usize {
        self.hand.cards().len()
    }

    pub fn discard_pile_len(&self) -> usize {
        self.discard_pile.cards().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::player::Player;

    #[test]
    fn player_intialization() {
        let player = Player::new("Alice");
        assert_eq!(player.name, "Alice");
        assert_eq!(player.points, 100);
        assert_eq!(player.hand_size(), 5);
        assert_eq!(player.deck_size(), 52 - player.hand_size());
        assert_eq!(player.discard_pile_len(), 0);
    }
}
