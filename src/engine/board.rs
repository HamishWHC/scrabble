const BOARD_SIZE_X: usize = 25;
const BOARD_SIZE_Y: usize = 25;
const MODIFIER_LOCATIONS: [(BoardLocation, SquareModifier); 1] =
    [(BoardLocation { x: 0, y: 0 }, SquareModifier::TripleWord)];

#[derive(Copy, Clone, PartialEq)]
pub struct BoardLocation {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone)]
pub struct Board {
    rows: [[Square; BOARD_SIZE_X]; BOARD_SIZE_Y],
}

impl Board {
    pub fn new() -> Self {
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

    pub fn get_square(&self, location: &BoardLocation) -> Option<Square> {
        if location.x >= BOARD_SIZE_X {
            return None;
        } else if location.y >= BOARD_SIZE_Y {
            return None;
        }
        Some(self.rows[location.y][location.x])
    }

    pub fn has_square(&self, location: &BoardLocation) -> bool {
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

    pub fn place_tile(&mut self, location: &BoardLocation, tile: Tile) -> () {
        let mut square = self.get_square(location).unwrap();
        square.tile = Some(tile);
        self.set_square(location, square)
    }

    pub fn render(&self) -> String {
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
pub struct Square {
    pub tile: Option<Tile>,
    pub modifier: Option<SquareModifier>,
}

#[derive(Copy, Clone, Debug)]
pub enum SquareModifier {
    TripleWord,
    DoubleWord,
    TripleLetter,
    DoubleLetter,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub letter: char,
    pub value: usize,
}