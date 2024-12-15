pub fn problem_1(data: &str) -> i64 {
    let mut state = State::parse(data);
    let num_instructions = state.instructions.len();
    for i in 0..num_instructions {
        // state.print_grid();
        // println!("next instruction: {:?}", state.instructions[i]);
        state.apply_instruction(i);
    }
    state.calculate_gps()
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Wall,
    Box,
    Empty,
}

#[derive(Clone, Copy, Debug)]
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
    fn parse(data: &str) -> State {
        let mut input: State = Default::default();
        let mut lines = data.lines().enumerate();
        for (r, line) in &mut lines {
            if line.is_empty() {
                break;
            }
            input.grid.push(
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '.' => Square::Empty,
                        '@' => {
                            input.pos = (r, c);
                            Square::Empty
                        }
                        'O' => Square::Box,
                        '#' => Square::Wall,
                        _ => panic!["invalid char {ch} in grid input"],
                    })
                    .collect(),
            );
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

        // Where we are at the start
        let current = self.pos;
        // Where we will potentially be at the end of this turn if there are no obstructions
        let next = instruction.next(current);
        match self.get_cell(next) {
            Square::Wall => {
                // hit a wall
                return;
            }
            Square::Empty => {
                // empty space just move in
            }
            Square::Box => {
                // we try to push the box or series of boxes.
                // this will only work if there is a line of boxes followed by an empty space.
                // pushing the boxes is equivalent to swapping the first box with this empty space
                let mut possibly_empty = next;
                while self.get_cell(possibly_empty) == Square::Box {
                    possibly_empty = instruction.next(possibly_empty);
                }
                if self.get_cell(possibly_empty) == Square::Wall {
                    return;
                }
                self.set_cell(next, Square::Empty);
                self.set_cell(possibly_empty, Square::Box);
            }
        }
        self.pos = next;
    }

    fn calculate_gps(&self) -> i64 {
        let mut gps = 0;
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell == Square::Box {
                    gps += r * 100 + c;
                }
            }
        }
        gps.try_into().unwrap()
    }

    fn print_grid(&self) {
        let mut s = String::new();
        for (r, row) in self.grid.iter().enumerate() {
            s.clear();
            for (c, square) in row.iter().enumerate() {
                if (r, c) == self.pos {
                    s.push('@');
                } else {
                    s.push(match square {
                        Square::Wall => '#',
                        Square::Box => 'O',
                        Square::Empty => '.',
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

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA_1, 2028),
        (test_problem_1_data_2, problem_1, DATA_2, 10092),
        (test_problem_2_data_1, problem_2, DATA_1, 0),
        (test_problem_2_data_2, problem_2, DATA_2, 0),
    );
}
