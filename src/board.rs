use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};

pub trait Board {
    fn reveal(&mut self, r: usize, c: usize);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

#[derive(Clone)]
pub enum TileState {
    // true if flagged, else false.
    HIDDEN(bool),
    // Value represents number of adjacent mines.
    // -1 means it's a mine.
    REVEALED(i8),
}

struct BoardImpl {
    tiles: Vec<Vec<TileState>>,
    mines: HashSet<(usize, usize)>,
    mines_count: usize,
}

impl Board for BoardImpl {
    fn reveal(&mut self, r: usize, c: usize) {
        if self.mines.is_empty() {
            // Unitialized board; place mines at random positions.
            let mut rng = thread_rng();
            let mut mine_free_tiles: Vec<(usize, usize)> = (0..self.get_width())
                .cartesian_product((0..self.get_height()))
                .collect();
            mine_free_tiles.shuffle(&mut rng);
            for p in mine_free_tiles.iter().take(self.mines_count) {
                self.mines.insert(*p);
            }
        }

        if self.mines.contains(&(r,c)) {
            self.tiles[r][c] = TileState::REVEALED(-1);
            return;
        }

        let mut neighbor_mines_count = 0;
        for rr in (max(r-1,0)..min(r+2,self.get_height())) {
            for cc in (max(c-1,0)..min(c+2,self.get_width())) {
                if rr == r && cc == c {
                    continue;
                }
                
                if let TileState::REVEALED(x) = self.tiles[rr][cc] {
                    if x == -1 {
                        neighbor_mines_count += 1;
                    }
                }
            }
        }
        
        self.tiles[r][c] = TileState::REVEALED(neighbor_mines_count);        
    }

    fn get_width(&self) -> usize {
        self.tiles.len()
    }

    fn get_height(&self) -> usize {
        self.tiles[0].len()
    }
}

pub fn new(width: usize, height: usize) -> Box<dyn Board> {
    let mut tiles = Vec::new();
    for r in 0..height {
        tiles.push(vec![TileState::HIDDEN(false); width]);
    }

    Box::new(BoardImpl {
        tiles: tiles,
        mines: HashSet::new(),
        mines_count: 10,
    })
}
