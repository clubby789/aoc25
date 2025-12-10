pub fn part1(input: &str) -> u64 {
    let mut input = input.as_bytes().to_vec();
    let grid = Grid::new(&mut input);
    grid.all_positions()
        .filter(|pos| grid.get(*pos) == Some(b'@'))
        .filter(|pos| {
            let result = grid.adjacent(pos).filter(|&c| c == b'@').count();
            result < 4
        })
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let mut input = input.as_bytes().to_vec();
    let mut grid = Grid::new(&mut input);
    let mut removed = 0;
    loop {
        let removable = grid
            .all_positions()
            .filter(|pos| grid.get(*pos) == Some(b'@'))
            .filter(|pos| {
                let result = grid.adjacent(pos).filter(|&c| c == b'@').count();
                result < 4
            })
            .collect::<Vec<_>>();
        if removable.is_empty() {
            break;
        } else {
            removed += removable.len();
            for pos in removable {
                grid.remove(pos);
            }
        }
    }
    removed as u64
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn adjacent_positions(&self) -> impl Iterator<Item = Self> + '_ {
        #[rustfmt::skip]
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),           (1, 0),
            (-1, 1),  (0, 1),  (1, 1)
        ];
        OFFSETS.iter().filter_map(|&(dx, dy)| {
            Some(Self {
                x: self.x.checked_add_signed(dx)?,
                y: self.y.checked_add_signed(dy)?,
            })
        })
    }
}

struct Grid<'a> {
    rows: Vec<&'a mut [u8]>,
    width: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a mut [u8]) -> Self {
        let rows = input.split_mut(|&c| c == b'\n').collect::<Vec<_>>();
        let width = rows[0].len();
        Self { rows, width }
    }

    fn adjacent<'pos>(&'a self, pos: &'pos Pos) -> impl Iterator<Item = u8> + 'pos
    where
        'a: 'pos,
    {
        pos.adjacent_positions().filter_map(|adj| self.get(adj))
    }

    fn get(&self, pos: Pos) -> Option<u8> {
        Some(*self.rows.get(pos.y)?.get(pos.x)?)
    }

    fn remove(&mut self, pos: Pos) {
        let cell = &mut self.rows[pos.y][pos.x];
        debug_assert_eq!(*cell, b'@');
        *cell = b'.'
    }

    fn all_positions(&self) -> impl Iterator<Item = Pos> {
        let mut pos = Pos { x: 0, y: 0 };
        let mut done = false;
        std::iter::from_fn(move || {
            if done {
                return None;
            };
            let result = Some(pos);
            if pos.x + 1 < self.width {
                pos.x += 1;
            } else if pos.y + 1 < self.rows.len() {
                pos.x = 0;
                pos.y += 1;
            } else {
                done = true;
            }
            result
        })
    }
}
