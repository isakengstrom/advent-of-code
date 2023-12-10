use std::hash::Hash;
use crate::position::Position;
use crate::tile::Tile;

#[derive(Debug, Clone)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub start_tile: Tile
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();

        let height = lines.len();
        let width = lines.first().unwrap().len();

        let mut start_position: Position = Position::init_empty();

        let mut tiles: Vec<Vec<Tile>> = Vec::with_capacity(height);
        for (height_index, line) in lines.iter().enumerate() {
            let mut tiles_row: Vec<Tile> = Vec::with_capacity(width);
            for (width_index, tile_char) in line.chars().enumerate() {
                let tile = Tile::new(tile_char, width_index as i32, height_index as i32);
                tiles_row.push(tile);
                if tile.is_start {
                    start_position = tile.position;
                }
            }
            tiles.push(tiles_row)
        }

        // Update start tile with connection information
        let start_tile = Self::set_start_tile_connections(start_position, tiles.clone());
        tiles[start_tile.position.y as usize][start_tile.position.x as usize] = start_tile;

        Maze { width, height, tiles, start_tile }
    }

    pub fn get_tile(&self, position: Position) -> Tile {
        self.tiles[position.y as usize][position.x as usize]
    }

    fn set_start_tile_connections(start_position: Position, tiles: Vec<Vec<Tile>>) -> Tile {
        let pos = start_position;
        let rows = tiles.len() as i32;
        let cols = tiles[0].len() as i32;

        let get_adjacent_tile = |dx, dy| {
            let new_x = pos.x + dx;
            let new_y = pos.y + dy;

            if new_x >= 0 && new_x < cols && new_y >= 0 && new_y < rows {
                Some(tiles[new_y as usize][new_x as usize].clone())
            } else {
                None
            }
        };

        let tile_to_left = get_adjacent_tile(-1, 0);
        let tile_to_right = get_adjacent_tile(1, 0);
        let tile_above = get_adjacent_tile(0, -1);
        let tile_below = get_adjacent_tile(0, 1);

        let connections = [
            (tile_to_left, vec!['-', 'L', 'F']),
            (tile_above, vec!['|', 'F', '7']),
            (tile_to_right, vec!['-', 'J', '7']),
            (tile_below, vec!['|', 'J', 'L']),
        ];

        let (connection_1, connection_2) = connections
            .iter()
            .flat_map(|(tile_option, pipe_structures)| {
                tile_option.filter(|tile| pipe_structures.contains(&tile.tile_sign))
            })
            .map(|tile| tile.position)
            .fold((Position::init_empty(), Position::init_empty()), |acc, position| {
                (position, acc.0)
            });

        Tile {
            is_pipe: true,
            is_start: true,
            tile_sign: 'S',
            position: pos,
            connection_1,
            connection_2,
        }
    }
}

