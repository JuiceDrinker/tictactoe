use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug)]
enum Result {
    Draw,
    Decisive(Player),
}

#[derive(Debug, PartialEq)]
enum TileOccupancy {
    Occupied(Player),
    Empty,
}

#[derive(Debug)]
struct Tile {
    tile_occupancy: TileOccupancy,
    label: u8,
}

#[derive(Debug)]
struct Game {
    result: Option<Result>,
    active_turn: Player,
    board: [Tile; 9],
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player = match self {
            Player::X => "X",
            Player::O => "O",
        };
        write!(f, "{}", player)
    }
}

impl Game {
    const WINNING_COMBININATIONS: [[u8; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    fn new() -> Self {
        Game {
            result: None,
            active_turn: Player::O,
            board: core::array::from_fn(|i| Tile {
                tile_occupancy: TileOccupancy::Empty,
                label: u8::try_from(i).unwrap(),
            }),
        }
    }
    fn make_move(&mut self, tile_index: u8) -> &mut Self {
        match Game::tile(self, tile_index) {
            Some(tile) => match tile.tile_occupancy {
                TileOccupancy::Occupied(_) => {
                    println!("That tile is already occupied, choose another move");
                    return self;
                }
                TileOccupancy::Empty => {
                    Game::set_tile(self, tile_index, TileOccupancy::Occupied(self.active_turn))
                }
            },
            None => {
                println!("{} is not a valid index, please choose again", tile_index);
                return self;
            }
        };

        self.active_turn = match self.active_turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        if self.is_game_drawn().is_some() {
            return self;
        }

        if let Some(result) = self.is_game_decisive() {
            self.result = Some(result);
            return self;
        }
        println!("Game won by {:?}", self.result);
        return self;
    }

    fn tile(&self, label: u8) -> Option<&Tile> {
        self.board.iter().find(|tile| tile.label == label)
    }

    fn set_tile(&mut self, label: u8, occupancy: TileOccupancy) -> () {
        if let Some(tile) = self.board.iter_mut().find(|tile| tile.label == label) {
            tile.tile_occupancy = occupancy
        }
    }

    fn is_game_decisive(&mut self) -> Option<Result> {
        let mut result: Option<Result> = None;
        for combos in Game::WINNING_COMBININATIONS {
            let mut x_tiles = 0;
            let mut o_tiles = 0;
            for c in combos {
                if let Some(owned) = Game::tile(&self, c) {
                    match owned.tile_occupancy {
                        TileOccupancy::Occupied(player) => match player {
                            Player::X => x_tiles = x_tiles + 1,
                            Player::O => o_tiles = o_tiles + 1,
                        },
                        TileOccupancy::Empty => continue,
                    }
                }
                if x_tiles == 3 {
                    result = Some(Result::Decisive(Player::X));
                }
                if o_tiles == 3 {
                    result = Some(Result::Decisive(Player::O));
                }
            }
        }
        return result;
    }

    fn is_game_drawn(&mut self) -> Option<&mut Game> {
        let is_game_drawn = match self.board.iter().all(|tile| match tile.tile_occupancy {
            TileOccupancy::Occupied(_) => true,
            _ => false,
        }) {
            true => true,
            false => false,
        };
        if is_game_drawn {
            self.result = Some(Result::Draw);
            return Some(self);
        }
        None
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tiles: String = String::from("\r\n");
        for (i, tile) in self.board.iter().enumerate() {
            tiles.push_str("|");
            match &tile.tile_occupancy {
                TileOccupancy::Occupied(player) => match player {
                    Player::X => tiles.push_str("X"),
                    Player::O => tiles.push_str("O"),
                },
                TileOccupancy::Empty => tiles.push_str(" "),
            }
            match i {
                // TODO: use if let here instead
                2 | 5 | 8 => tiles.push_str(" \r\n"),
                _ => continue,
            }
        }

        write!(f, "{}", tiles)
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe");
    let mut game = Game::new();

    while game.result.is_none() {
        print!("Current game: {}", &game);
        print!("It is {}'s turn to play \r\n", &game.active_turn);
        println!(
            "{}, input number 1 - 9 to make your move",
            &game.active_turn
        );
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let parsed_input: u8 = match input.trim().parse::<u8>() {
            Ok(num) => {
                if num > 0 {
                    num - 1
                } else {
                    0
                }
            }
            Err(_) => {
                println!("Thats not a valid number, please try agan");
                continue;
            }
        };
        Game::make_move(&mut game, parsed_input);
        if let Some(result) = &game.result {
            match &result {
                Result::Draw => {
                    println!("Game ended in a draw");
                    break;
                }
                Result::Decisive(player) => {
                    println!("{} won! Congratulations", player);
                }
            }
        }
    }
}
