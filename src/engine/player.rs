use super::board::{Board, Tile};
use super::game::{Game, Word};

pub struct Player {
    pub hand: Vec<Tile>,
    pub score: usize
}

impl Player {
    pub fn new(game: &mut Game) -> Self {
        let mut new_player = Player {
            score: 0,
            hand: vec![]
        };

        new_player.hand = game.bag.draw_tiles(7);

        new_player
    }

    pub fn play_word(&mut self, word: &Word, board: &Board) -> () {}

    pub fn remove_tiles(&mut self, indexes: Vec<usize>) -> Vec<Tile> {
        let hand_size = self.hand.len();
        let mut indexes = indexes.clone();
        indexes.sort(); // Needs to be sorted so that removing multiple tiles doesn't change where the tile to discard is.
        indexes.reverse();
        let mut removed: Vec<Tile> = vec![];
        for i in indexes.iter() {
            if i < &hand_size {
                removed.push(self.hand.remove(*i));
            }
        }
        removed
    }
}