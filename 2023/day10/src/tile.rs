use crate::position::Position;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub is_pipe: bool,
    pub is_start: bool,
    pub tile_sign: char,
    pub position: Position,
    pub connection_1: Position,
    pub connection_2: Position
}

impl Tile {
    pub fn new(tile_char: char, width: i32, height: i32) -> Self {
        if tile_char == '.' {
            let tile = Tile {
                is_pipe: false,
                is_start: false,
                tile_sign: tile_char,
                position: Position::init_empty(),
                connection_1: Position::init_empty(),
                connection_2: Position::init_empty(),
            };

            tile
        }
        else if tile_char == 'S' {
            let tile = Tile {
                is_pipe: true,
                is_start: true,
                tile_sign: tile_char,
                position: Position::new(width, height),
                connection_1: Position::init_empty(),
                connection_2: Position::init_empty(),
            };

            tile
        }
        else {
            let (connection_1, connection_2) = match tile_char {
                'J' => (Position::new(width+0, height-1), Position::new(width-1, height+0)),
                'F' => (Position::new(width+1, height+0), Position::new(width+0, height+1)),
                '7' => (Position::new(width-1, height+0), Position::new(width+0, height+1)),
                'L' => (Position::new(width+0, height-1), Position::new(width+1, height+0)),
                '-' => (Position::new(width-1, height+0), Position::new(width+1, height+0)),
                '|' => (Position::new(width+0, height-1), Position::new(width+0, height+1)),
                _ => panic!("Unexpected tile character: {}", tile_char),
            };

            let tile = Tile {
                is_pipe: true,
                is_start: false,
                tile_sign: tile_char,
                position: Position::new(width, height),
                connection_1,
                connection_2,
            };

            tile
        }
    }
}
