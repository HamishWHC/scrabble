use rand::Rng;

const BOARD_SIZE_X: usize = 25;
const BOARD_SIZE_Y: usize = 25;
const MODIFIER_LOCATIONS: [(BoardLocation, SquareModifier); 1] =
    [(BoardLocation { x: 0, y: 0 }, SquareModifier::TripleWord)];

#[derive(Copy, Clone)]
struct BoardLocation {
    x: usize,
    y: usize,
}

struct Word {
    tiles: Vec<(BoardLocation, Tile)>
}

impl Word {
    fn new(tiles: Vec<(BoardLocation, Tile)>, board: &Board) -> Result<Self, &'static str> {
        if tiles.len() == 0 {return Err("Word must contain at least one tile.")}

        let existing_adjacency = false;
        let mut x = Some(tiles[0].0.x);
        let mut y = Some(tiles[0].0.y);
        for tile in tiles {
            if x != None && x.unwrap() != tile.0.x {
                x = None;
            }
            if y != None && y.unwrap() != tile.0.y {
                y = None;
            }
            if y == None && x == None {
                return Err("Word must be on one line (horizontally or vertically)!")
            }

            let mut adjacency_check = false;
            for t in tiles {
                if !adjacency_check {
                    if x != None {
                        if t.0.x == tile.0.x + 1 || t.0.x == tile.0.x - 1 {
                            adjacency_check = true;
                        } else if  {

                        }
                    } else if y != None {
    
                    }
                }
            }
        }

        Ok(Word {
            tiles: tiles
        })
    }
}

enum TurnAction {
    PlayWord(Word),
    DiscardTiles(Vec<usize>)
}

struct Player {
    hand: Vec<Tile>,
    score: usize
}

impl Player {
    fn draw_tiles(&mut self, bag: &mut Vec<Tile>) -> () {
        let mut rng = rand::thread_rng();
        let mut bag_count = bag.len();
        while self.hand.len() < 7 {
            let chosen = rng.gen_range(0, bag_count);
            let tile = bag.remove(chosen);
            bag_count -= 1;
            self.hand.push(tile);
        }
    }

    fn play_word(&mut self, word: &Word, board: &Board, ) -> () {}

    fn discard_tiles(&mut self, indexes: &Vec<usize>, bag: &Bag) -> () {
        let hand_size = self.hand.len();
        let mut indexes = indexes.clone();
        indexes.sort(); // Needs to be sorted so that removing multiple tiles doesn't change where the tile to discard is.
        indexes.reverse();
        for i in indexes.iter() {
            if hand_size < *i {
                bag.contents.push(self.hand.remove(*i));
            }
        }
    }

    fn take_turn(&mut self, action: &TurnAction, game: &mut Game) -> () {
        match action {
            TurnAction::PlayWord(x) => {
                self.play_word(x, &game.board)
            },
            TurnAction::DiscardTiles(x) => {
                self.discard_tiles(x, &game.bag);
            }
        }
        self.hand.extend(game.bag.draw_random_many(7-self.hand.len()));
    }
}

struct Game {
    board: Board,
    bag: Bag
}

struct Bag {
    contents: Vec<Tile>
}

impl Bag {
    fn draw_random_one(&mut self) -> Option<Tile> {
        let count = self.contents.len();
        if count > 0 {
            let rng = rand::thread_rng();
            let chosen = rng.gen_range(0, count);
            Some(self.contents.remove(chosen))
        } else {
            None
        }
    }

    fn draw_random_many(&mut self, count: usize) -> Vec<Tile> {
        let rng = rand::thread_rng();
        let mut bag_count = self.contents.len();
        let chosen: Vec<Tile> = vec![];
        let chosen_count = chosen.len();
        while chosen_count < count && bag_count > 0 {
            let index = rng.gen_range(0, bag_count);
            chosen.push(self.contents.remove(index));
            bag_count -= 1;
            chosen_count += 1;
        }
        chosen
    }
}

#[derive(Copy, Clone)]
struct Board {
    rows: [[Square; BOARD_SIZE_X]; BOARD_SIZE_Y],
}

impl Board {
    fn new() -> Self {
        let mut board = Self {
            rows: [[Square {
                tile: None,
                modifier: None,
            }; BOARD_SIZE_X]; BOARD_SIZE_Y],
        };

        for location in MODIFIER_LOCATIONS.iter() {
            board.rows[location.0.y][location.0.x].modifier = Some(location.1);
        }

        board
    }

    fn get_square(&self, location: &BoardLocation) -> &Square {
        &self.rows[location.y][location.x]
    }

    fn set_square(&mut self, location: &BoardLocation, square: Square) -> () {
        self.rows[location.y][location.x] = square;
    }

    fn place_tile(&mut self, location: &BoardLocation, tile: Tile) -> () {
        let mut square = *self.get_square(location);
        square.tile = Some(tile);
        self.set_square(location, square)
    }

    fn render(&self) -> String {
        let mut rendered_board = {
            let mut header = String::from("  ");
            for x in 0..BOARD_SIZE_X {
                header.push_str(&format!("|{:2}", x + 1)[..])
            }
            header
        };

        let row_separator: String = {
            let mut sep = String::from("--");
            for _ in 0..BOARD_SIZE_X {
                sep.push_str(&format!("+--",)[..])
            }
            sep
        };

        for (y, row) in self.rows.iter().enumerate() {
            rendered_board.push_str(
                &format!(
                    "\n{}\n{:>2}",
                    row_separator,
                    (65 + y) as u8 as char // Offsets integer to start at A and converts to character.
                )[..], // Converts to string slice (and provides reference with & operator).
            );
            for square in row.iter() {
                rendered_board.push_str("|");
                match square.tile {
                    Some(x) => rendered_board.push_str(&format!("{}{}", x.letter, x.value)[..]),
                    None => rendered_board.push_str(match square.modifier {
                        Some(x) => match x {
                            SquareModifier::DoubleLetter => "DL",
                            SquareModifier::TripleLetter => "TL",
                            SquareModifier::DoubleWord => "DW",
                            SquareModifier::TripleWord => "TW",
                        },
                        None => "  ",
                    })
                };
            }
        }
        rendered_board
    }
}

#[derive(Copy, Clone, Debug)]
struct Square {
    tile: Option<Tile>,
    modifier: Option<SquareModifier>,
}

#[derive(Copy, Clone, Debug)]
enum SquareModifier {
    TripleWord,
    DoubleWord,
    TripleLetter,
    DoubleLetter,
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    letter: char,
    value: usize,
}

fn main() {
    let mut game = Game {
        board: Board::new(),
        bag: vec![]
    };

    let mut players = vec![Player {
        hand: vec![Tile {letter: 'A', value: 1}]
    }];

    game.board.place_tile(&BoardLocation {x: 0, y: 1}, Tile {letter: 'B', value: 3});

    println!("{}", game.board.render());
}
