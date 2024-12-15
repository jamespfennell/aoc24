use std::collections::HashSet;

pub fn problem_1(data: &str) -> i64 {
    run(data, false)
}

pub fn problem_2(data: &str) -> i64 {
    run(data, true)
}

pub fn run(data: &str, doubling_mode: bool) -> i64 {
    let mut state = State::parse(data, doubling_mode);
    let num_instructions = state.instructions.len();
    for i in 0..num_instructions {
        // state.print_grid();
        // println!("next instruction: {:?}", state.instructions[i]);
        state.apply_instruction(i);
    }
    // state.print_grid();
    state.calculate_gps()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Box,
    LeftBox,
    RightBox,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn next(&self, (r, c): (usize, usize)) -> (usize, usize) {
        match self {
            Instruction::Up => (r - 1, c),
            Instruction::Down => (r + 1, c),
            Instruction::Left => (r, c - 1),
            Instruction::Right => (r, c + 1),
        }
    }
}

#[derive(Default, Debug)]
struct State {
    grid: Vec<Vec<Square>>,
    pos: (usize, usize),
    instructions: Vec<Instruction>,
}

impl State {
    fn parse(data: &str, doubling_mode: bool) -> State {
        let mut input: State = Default::default();
        let mut lines = data.lines().enumerate();
        for (r, line) in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut row: Vec<Square> = vec![];
            for (c, ch) in line.chars().enumerate() {
                let square = match ch {
                    '.' => Square::Empty,
                    '@' => {
                        input.pos = (r, if doubling_mode { c * 2 } else { c });
                        Square::Empty
                    }
                    'O' => {
                        if doubling_mode {
                            row.push(Square::LeftBox);
                            row.push(Square::RightBox);
                            continue;
                        }
                        Square::Box
                    }
                    '#' => Square::Wall,
                    _ => panic!["invalid char {ch} in grid input"],
                };
                row.push(square);
                if doubling_mode {
                    row.push(square);
                }
            }
            input.grid.push(row);
        }
        for (_, line) in lines {
            for c in line.chars() {
                input.instructions.push(match c {
                    '^' => Instruction::Up,
                    '>' => Instruction::Right,
                    'v' => Instruction::Down,
                    '<' => Instruction::Left,
                    '\n' => continue,
                    _ => panic!["invalid char {c} in instructions input"],
                });
            }
        }
        input
    }

    fn get_cell(&self, (r, c): (usize, usize)) -> Square {
        self.grid[r][c]
    }

    fn set_cell(&mut self, (r, c): (usize, usize), square: Square) {
        self.grid[r][c] = square;
    }

    fn apply_instruction(&mut self, i: usize) {
        let instruction = self.instructions[i];
        let mut to_shift: Vec<HashSet<(usize, usize)>> = vec![[self.pos].into()];
        loop {
            // check if we can shift the cells in the last row up.
            let mut new_cs: HashSet<(usize, usize)> = Default::default();
            for &prev in to_shift.last().unwrap() {
                let next = instruction.next(prev);
                match self.get_cell(next) {
                    Square::Wall => {
                        // If there is any wall blocking, can't push.
                        return;
                    }
                    Square::Empty => {}
                    Square::Box => {
                        new_cs.insert(next);
                    }
                    Square::LeftBox => {
                        new_cs.insert(next);
                        if instruction == Instruction::Down || instruction == Instruction::Up {
                            new_cs.insert((next.0, next.1 + 1));
                        }
                    }
                    Square::RightBox => {
                        new_cs.insert(next);
                        if instruction == Instruction::Down || instruction == Instruction::Up {
                            new_cs.insert((next.0, next.1 - 1));
                        }
                    }
                }
            }
            // If there are no cells in the next row that will push upwards/downwards,
            // we are done and can move the thing.
            if new_cs.is_empty() {
                break;
            }
            to_shift.push(new_cs);
        }

        // If we made it to here, we can push.
        while let Some(cs) = to_shift.pop() {
            for source in cs {
                let sq_to_move = self.get_cell(source);
                let target = instruction.next(source);
                self.set_cell(target, sq_to_move);
                self.set_cell(source, Square::Empty);
            }
        }
        self.pos = instruction.next(self.pos);
    }

    fn calculate_gps(&self) -> i64 {
        let mut gps = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == Square::Box || *cell == Square::LeftBox {
                    gps += r * 100 + c;
                }
            }
        }
        gps.try_into().unwrap()
    }

    fn _print_grid(&self) {
        let mut s = String::new();
        for (r, row) in self.grid.iter().enumerate() {
            s.clear();
            for (c, square) in row.iter().enumerate() {
                if (r, c) == self.pos {
                    s.push('@');
                } else {
                    s.push(match square {
                        Square::Wall => '#',
                        Square::Empty => '.',
                        Square::Box => '0',
                        Square::LeftBox => '[',
                        Square::RightBox => ']',
                    });
                }
            }
            println!("{s}");
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const DATA_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const DATA_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA_1, 2028),
        (test_problem_1_data_2, problem_1, DATA_2, 10092),
        (test_problem_2_data_2, problem_2, DATA_2, 9021),
        (test_problem_2_data_3, problem_2, DATA_3, 618),
    );
}
