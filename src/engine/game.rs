use super::board::{Board, Tile, BoardLocation};
use rand::Rng;

const DEFAULT_BAG: [(char, usize, usize); 27] = [
    (' ', 0, 2),
    ('E', 1, 12),
    ('A', 1, 9),
    ('I', 1, 9),
    ('O', 1, 8),
    ('N', 1, 6),
    ('R', 1, 6),
    ('T', 1, 6),
    ('L', 1, 4),
    ('S', 1, 4),
    ('U', 1, 4),
    ('D', 2, 4),
    ('G', 2, 3),
    ('B', 3, 2),
    ('C', 3, 2),
    ('M', 3, 2),
    ('P', 3, 2),
    ('F', 4, 2),
    ('H', 4, 2),
    ('V', 4, 2),
    ('W', 4, 2),
    ('Y', 4, 2),
    ('K', 5, 1),
    ('J', 8, 1),
    ('X', 8, 1),
    ('Q', 10, 1),
    ('Z', 10, 1),
];

pub struct Word {
    pub tiles: Vec<(BoardLocation, Tile)>,
}

impl Word {
    pub fn new(placements: Vec<(BoardLocation, Tile)>, board: &Board) -> Result<Self, &'static str> {
        if placements.len() == 0 {
            return Err("Word must contain at least one tile.");
        }

        let first_placement = placements[0].0;
        let mut valid_x = true;
        let mut valid_y = true;
        for placement in &placements {
            if valid_x && first_placement.x != placement.0.x {
                valid_x = false;
            }
            if valid_y && first_placement.y != placement.0.y {
                valid_y = false;
            }
            if !valid_x && !valid_y {
                return Err("Word must be on one line (horizontally or vertically)!");
            }

            match board.get_square(&BoardLocation {
                x: placement.0.x,
                y: placement.0.y,
            }) {
                Some(sq) => match sq.tile {
                    Some(_) => return Err("Cannot place tile on an existing tile!"),
                    None => {}
                },
                None => {}
            }

            let mut valid_adjacency = false;

            if valid_x {
                let valid_left = {
                    let mut valid = false;
                    for p in &placements {
                        if !valid && p.0.y == placement.0.y - 1 {
                            valid = true;
                        }
                    }
                    valid
                } || board.has_square(&BoardLocation {
                    x: placement.0.x,
                    y: placement.0.y - 1,
                });
                let valid_right = {
                    let mut valid = false;
                    for p in &placements {
                        if !valid && p.0.y == placement.0.y + 1 {
                            valid = true;
                        }
                    }
                    valid
                } || board.has_square(&BoardLocation {
                    x: placement.0.x,
                    y: placement.0.y + 1,
                });
                if valid_left || valid_right {
                    valid_adjacency = true;
                }
            }
            if valid_y {
                let valid_up = {
                    let mut valid = false;
                    for p in &placements {
                        if !valid && p.0.x == placement.0.x - 1 {
                            valid = true;
                        }
                    }
                    valid
                } || board.has_square(&BoardLocation {
                    x: placement.0.x - 1,
                    y: placement.0.y,
                });
                let valid_down = {
                    let mut valid = false;
                    for p in &placements {
                        if !valid && p.0.x == placement.0.x + 1 {
                            valid = true;
                        }
                    }
                    valid
                } || board.has_square(&BoardLocation {
                    x: placement.0.x + 1,
                    y: placement.0.y,
                });
                if valid_up || valid_down {
                    valid_adjacency = true;
                }
            }
            if !valid_adjacency {
                return Err("Tile is not adjacent to any other tile.")
            }
        }

        Ok(Word { tiles: placements })
    }
}

pub enum TurnAction {
    PlayWord(Word),
    DiscardTiles(Vec<usize>),
}

pub struct Game {
    pub board: Board,
    pub bag: Bag,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            bag: Bag::new(false)
        }
    }
}

pub struct Bag {
    pub contents: Vec<Tile>,
}

impl Bag {
    pub fn new(empty: bool) -> Self {
        let mut bag = Bag {contents: vec![]};
        if !empty {
            for tile in DEFAULT_BAG.iter() {
                bag.contents.extend((0..tile.2).map(|_| Tile {letter: tile.0, value: tile.1}))
            }
            bag
        } else {
            bag
        }
        
    }

    pub fn draw_tiles(&mut self, count: usize) -> Vec<Tile> {
        let mut rng = rand::thread_rng();
        let mut bag_count = self.contents.len();
        let mut chosen: Vec<Tile> = vec![];
        let mut chosen_count = chosen.len();
        while chosen_count < count && bag_count > 0 {
            let index = rng.gen_range(0, bag_count);
            chosen.push(self.contents.remove(index));
            bag_count -= 1;
            chosen_count += 1;
        }
        chosen
    }

    pub fn return_tiles(&mut self, tiles: Vec<Tile>) -> () {
        self.contents.extend(tiles);
    }
}