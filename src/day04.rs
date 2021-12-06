pub fn run(input: &str) {
    let (marks, boards) = parse(input);
    println!("day 04 - part 1: {:?}", crate::rough_time(|| part_1(&marks, boards.clone())));
    println!("day 04 - part 2: {:?}", crate::rough_time(|| part_2(&marks, boards)));
}

fn parse(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.trim().lines().filter(|s| !s.is_empty());
    let marks = lines.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    let mut boards = Vec::new();
    let mut current_board = Board::default();
    loop {
        let mut has_contents = false;
        let groups = lines.by_ref().take(5);
        for (i, group) in groups.enumerate() {
            has_contents = true;
            let numbers = group.split(' ').filter(|s| !s.is_empty()).map(|s| (false, s.parse().unwrap()));
            for (default, parsed) in current_board.tiles[i].iter_mut().zip(numbers) {
                *default = parsed;
            }
        }

        if !has_contents {
            break;
        }

        boards.push(core::mem::take(&mut current_board));
    }

    (marks, boards)
}

#[derive(Default, Clone, Copy)]
struct Board {
    tiles: [[(bool, u8); 5]; 5],
}

impl Board {
    fn mark(&mut self, value: u8) {
        if let Some((marked, _)) =
            self.tiles.iter_mut().flat_map(|tiles| tiles.iter_mut()).find(|(_, tile)| *tile == value)
        {
            *marked = true;
        }
    }

    fn check(&self) -> bool {
        let any_row_marked = || self.tiles.iter().any(|tiles| tiles.iter().all(|(marked, _)| *marked));
        let any_column_marked = || {
            (0..5)
                .map(|i| {
                    [self.tiles[0][i].0, self.tiles[1][i].0, self.tiles[2][i].0, self.tiles[3][i].0, self.tiles[4][i].0]
                })
                .any(|tiles| tiles.into_iter().all(core::convert::identity))
        };

        any_row_marked() || any_column_marked()
    }

    fn unmarked(&self) -> impl Iterator<Item = u8> + '_ {
        self.tiles.iter().flat_map(|tiles| tiles.iter()).copied().filter(|(marked, _)| !marked).map(|(_, tile)| tile)
    }
}

fn part_1(marks: &[u8], mut boards: Vec<Board>) -> u32 {
    let mut marks = marks.iter().copied();

    for mark in marks.by_ref().take(5) {
        for board in &mut boards {
            board.mark(mark);
        }
    }

    for mark in marks {
        for board in &mut boards {
            board.mark(mark);
            if board.check() {
                return board.unmarked().fold(0u32, |sum, tile| sum + tile as u32) * mark as u32;
            }
        }
    }

    unreachable!()
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.tiles {
            for (i, tile) in row.iter().enumerate() {
                write!(
                    f,
                    "{prefix}{bold}{:>2}\x1B[0m",
                    tile.1,
                    prefix = if i == 0 { "" } else { " " },
                    bold = if tile.0 { "\x1B[1m" } else { "" },
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[test]
fn part_1_test() {
    let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    let (marks, boards) = parse(input);
    assert_eq!(part_1(&marks, boards), 4512);
}

fn part_2(marks: &[u8], mut unwon_boards: Vec<Board>) -> u32 {
    let mut marks = marks.iter().copied();

    for mark in marks.by_ref().take(5) {
        for board in &mut unwon_boards {
            board.mark(mark);
        }
    }

    let mut to_removes = Vec::with_capacity(unwon_boards.len());
    for mark in marks {
        let len = unwon_boards.len();
        for (i, board) in unwon_boards.iter_mut().enumerate() {
            board.mark(mark);
            let won = board.check();
            if won && len != 1 {
                to_removes.push(i);
            } else if won {
                return board.unmarked().fold(0u32, |sum, tile| sum + tile as u32) * mark as u32;
            }
        }

        for index in to_removes.drain(..).rev() {
            unwon_boards.remove(index);
        }
    }

    unreachable!()
}
