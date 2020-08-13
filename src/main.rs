use rand::Rng;
use std::io;

const BOARD_SIZE_X: usize = 25;
const BOARD_SIZE_Y: usize = 25;
const MODIFIER_LOCATIONS: [(BoardLocation, SquareModifier); 1] =
    [(BoardLocation { x: 0, y: 0 }, SquareModifier::TripleWord)];

#[derive(Copy, Clone, PartialEq)]
struct BoardLocation {
    x: usize,
    y: usize,
}

struct Word {
    tiles: Vec<(BoardLocation, Tile)>,
}

impl Word {
    fn new(placements: Vec<(BoardLocation, Tile)>, board: &Board) -> Result<Self, &'static str> {
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

enum TurnAction {
    PlayWord(Word),
    DiscardTiles(Vec<usize>),
}

struct Player {
    hand: Vec<Tile>,
    score: usize
}

impl Player {
    fn new(game: &mut Game) -> Self {
        let mut new_player = Player {
            score: 0,
            hand: vec![]
        };

        new_player.hand = game.bag.draw_tiles(7);

        new_player
    }

    fn play_word(&mut self, word: &Word, board: &Board) -> () {}

    fn remove_tiles(&mut self, indexes: Vec<usize>) -> Vec<Tile> {
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

struct Game {
    board: Board,
    bag: Bag,
}

struct Bag {
    contents: Vec<Tile>,
}

impl Bag {
    fn draw_tiles(&mut self, count: usize) -> Vec<Tile> {
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

    fn return_tiles(&mut self, tiles: Vec<Tile>) -> () {
        self.contents.extend(tiles);
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

    fn get_square(&self, location: &BoardLocation) -> Option<Square> {
        if location.x >= BOARD_SIZE_X {
            return None;
        } else if location.y >= BOARD_SIZE_Y {
            return None;
        }
        Some(self.rows[location.y][location.x])
    }

    fn has_square(&self, location: &BoardLocation) -> bool {
        match self.get_square(location) {
            Some(sq) => match sq.tile {
                Some(_) => true,
                None => false,
            },
            None => false,
        }
    }

    fn set_square(&mut self, location: &BoardLocation, square: Square) -> () {
        self.rows[location.y][location.x] = square;
    }

    fn place_tile(&mut self, location: &BoardLocation, tile: Tile) -> () {
        let mut square = self.get_square(location).unwrap();
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
                    }),
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
    println!("Welcome to Command Line Scrabble!");
    println!("=================================");
    println!("Enter player count (2 to 4): ");

    let player_count: usize;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<usize>() {
            Ok(i) => {
                if i > 4 || i < 2 {
                    println!("Enter number from 2 to 4!")
                } else {
                    player_count = i;
                    println!("Creating game.");
                    break;
                }
            },
            Err(_) => {println!("Invalid number of players!")}
        };
    }

    let mut game = Game {
        board: Board::new(),
        bag: Bag { contents: vec![
            Tile {letter: 'b', value: 5},
            Tile {letter: 'c', value: 5},
            Tile {letter: 'd', value: 5},
            Tile {letter: 'e', value: 5},
            Tile {letter: 'f', value: 5},
            Tile {letter: 'g', value: 5},
            Tile {letter: 'h', value: 5},
            Tile {letter: 'i', value: 5},
            Tile {letter: 'j', value: 5},
            Tile {letter: 'k', value: 5},
            Tile {letter: 'l', value: 5}
        ] },
    };

    let mut players: Vec<Player> = vec![];

    for _ in 0..player_count {
        let new_player = Player::new(&mut game);
        players.push(new_player);
    }

    // game.board.place_tile(
    //     &BoardLocation { x: 0, y: 1 },
    //     Tile {
    //         letter: 'B',
    //         value: 3,
    //     },
    // );
    // println!("{}", game.board.render());

    let mut current_player_number = 0;

    loop {
        let current_player = match players.get_mut(current_player_number) {
            Some(p) => p,
            None => continue
        };

        let action = TurnAction::DiscardTiles(vec![0, 1, 2]);

        match action {
            TurnAction::PlayWord(x) => current_player.play_word(&x, &game.board),
            TurnAction::DiscardTiles(x) => {
                game.bag.return_tiles(current_player.remove_tiles(x));
            }
        }
        current_player.hand.extend(game.bag.draw_tiles(7 - current_player.hand.len()));

        current_player_number += 1;
        if current_player_number == player_count {
            current_player_number = 0;
        }

        for player in players {
            println!("{:#?}", player.hand)
        }

        break;
    }
}
