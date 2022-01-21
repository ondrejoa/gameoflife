use super::Position;
use bevy::prelude::*;

pub struct CellMatrix {
    width: u32,
    height: u32,
    cells: Vec<CellType>,
}

impl CellMatrix {
    pub fn new<'a, I>(width: u32, height: u32, cells: I) -> Self
    where
        I: Iterator<Item = (Entity, &'a Position)>,
    {
        let mut cm = Self {
            width,
            height,
            cells: vec![CellType::None; (width * height) as usize],
        };
        cm.push_all(cells);
        cm
    }

    fn push_all<'a, I>(&mut self, cells: I)
    where
        I: Iterator<Item = (Entity, &'a Position)>,
    {
        for (entity, position) in cells {
            self.cells[(position.x + self.width as i32 * position.y) as usize] =
                match self.cells[(position.x + self.width as i32 * position.y) as usize] {
                    CellType::Live(_, _) => {
                        panic!()
                    }
                    CellType::Dead(nc) => CellType::Live(entity, nc),
                    CellType::None => CellType::Live(entity, 0),
                };
            for position in CellMatrix::get_neighbours(position).iter().filter(|p| {
                p.x >= 0 && p.y >= 0 && p.x < self.width as i32 && p.y < self.height as i32
            }) {
                self.cells[(position.x + self.width as i32 * position.y) as usize] =
                    match self.cells[(position.x + self.width as i32 * position.y) as usize] {
                        CellType::Live(entity, nc) => CellType::Live(entity, nc + 1),
                        CellType::Dead(nc) => CellType::Dead(nc + 1),
                        CellType::None => CellType::Dead(1),
                    };
            }
        }
    }

    fn get_neighbours(position: &Position) -> Vec<Position> {
        let mut ps = Vec::with_capacity(8);
        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                ps.push(Position {
                    x: position.x + x,
                    y: position.y + y,
                });
            }
        }
        ps
    }

    pub fn live_cells(&self) -> impl Iterator<Item = (Entity, u32)> + '_ {
        CellIterator {
            cm: self,
            x: 0,
            y: 0,
        }
        .filter_map(|(x, y)| {
            if let CellType::Live(entity, nc) = self.cells[(x + self.width as i32 * y) as usize] {
                Some((entity, nc))
            } else {
                None
            }
        })
    }

    pub fn dead_cells(&self) -> impl Iterator<Item = (Position, u32)> + '_ {
        CellIterator {
            cm: self,
            x: 0,
            y: 0,
        }
        .filter_map(|(x, y)| {
            if let CellType::Dead(nc) = self.cells[(x + self.width as i32 * y) as usize] {
                Some((Position { x, y }, nc))
            } else {
                None
            }
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum CellType {
    Live(Entity, u32),
    Dead(u32), // miafasz
    None,
}

struct CellIterator<'a> {
    cm: &'a CellMatrix,
    x: i32,
    y: i32,
}

impl Iterator for CellIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.cm.height as i32 {
            None
        } else {
            let item = (self.x, self.y);
            self.x = (self.x + 1) % self.cm.width as i32;
            if self.x == 0 {
                self.y += 1;
            }
            Some(item)
        }
    }
}
