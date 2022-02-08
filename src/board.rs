use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::iter::Iterator;

pub trait Board {
    fn reveal(&mut self, r: usize, c: usize);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn debug_print(&self);
}

#[derive(Clone, PartialEq)]
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
            self.setup_mines((r, c));
        }

        if self.mines.contains(&(r, c)) {
            self.tiles[r][c] = TileState::REVEALED(-1);
            return;
        }

        let mut neighbor_mines_count = 0;
        for (rr, cc) in self.get_neighbors((r, c)) {
            if rr == r && cc == c {
                continue;
            }

            if self.mines.contains(&(rr, cc)) {
                neighbor_mines_count += 1;
            }
        }

        self.tiles[r][c] = TileState::REVEALED(neighbor_mines_count);

        self.expand_revealed_tile((r, c))
    }

    fn get_width(&self) -> usize {
        self.tiles.len()
    }

    fn get_height(&self) -> usize {
        self.tiles[0].len()
    }

    fn debug_print(&self) {
        let mut s = String::new();
        for r in 0..self.get_height() {
            for c in 0..self.get_width() {
                match self.tiles[r][c] {
                    TileState::HIDDEN(_) => s.push_str("█"),
                    TileState::REVEALED(0) => s.push_str(" "),
                    TileState::REVEALED(-1) => s.push_str("*"),
                    TileState::REVEALED(x) => s.push_str(&x.to_string()),
                }
            }
            s.push_str("\n");
        }
        println!("{}", s);
    }
}

impl BoardImpl {
    fn setup_mines(&mut self, exclude: (usize, usize)) {
        // Unitialized board; place mines at random positions.
        let mut rng = thread_rng();
        let mut mine_free_tiles: Vec<(usize, usize)> = (0..self.get_width())
            .cartesian_product(0..self.get_height())
            .filter(|&p| p != exclude)
            .collect();
        mine_free_tiles.shuffle(&mut rng);
        for p in mine_free_tiles.iter().take(self.mines_count) {
            self.mines.insert(*p);
        }
    }

    fn expand_revealed_tile(&mut self, tile: (usize, usize)) {
        // Use DFS.
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut s: HashSet<(usize, usize)> = HashSet::new();
        q.push_back(tile);

        while !q.is_empty() {
            let (r, c) = q.pop_back().unwrap();

            match self.tiles[r][c] {
                TileState::REVEALED(0) => (),
                TileState::REVEALED(_) => continue,
                TileState::HIDDEN(_) => {
                    if !self.mines.contains(&(r, c)) {
                        self.tiles[r][c] =
                            TileState::REVEALED(self.num_neighbor_mines((r, c)) as i8);
                    }
                }
            }

            for neighbor in self.get_neighbors((r, c)) {
                if !s.contains(&neighbor) {
                    q.push_back(neighbor);
                    s.insert(neighbor);
                }
            }
        }
    }

    fn get_neighbors(&self, tile: (usize, usize)) -> Vec<(usize, usize)> {
        let from_r = max((tile.0 as i64) - 1, 0) as usize;
        let to_r = min(tile.0 + 2, self.get_height());
        let from_c = max((tile.1 as i64) - 1, 0) as usize;
        let to_c = min(tile.1 + 2, self.get_width());

        (from_r..to_r)
            .cartesian_product(from_c..to_c)
            .filter(|&p| p != tile)
            .collect()
    }

    fn num_neighbor_mines(&self, tile: (usize, usize)) -> u8 {
        let mut mines_count = 0;
        for n in self.get_neighbors(tile) {
            if self.mines.contains(&n) {
                mines_count += 1;
            }
        }

        mines_count
    }
}

pub fn new(width: usize, height: usize, mines_count: usize) -> Option<Box<dyn Board>> {
    if mines_count >= height * width {
        return None;
    }

    let mut tiles = Vec::new();
    for _ in 0..height {
        tiles.push(vec![TileState::HIDDEN(false); width]);
    }

    Some(Box::new(BoardImpl {
        tiles: tiles,
        mines: HashSet::new(),
        mines_count: mines_count,
    }))
}
