use super::iter::Neighbors;
use std::collections::HashMap;

pub fn problem_1(data: &str) -> i64 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let regions = build_regions(&grid);
    let mut sum = 0_i64;
    for region in &regions {
        let mut perimeter = 0_i64;
        let mut area = 0_i64;
        for (r, c) in region.iter().copied() {
            area += 1;
            perimeter += 4;
            let plant = grid[r][c];
            for (r, c) in Neighbors::new(&grid, r, c) {
                if grid[r][c] == plant {
                    perimeter -= 1;
                }
            }
        }
        sum += area * perimeter;
    }
    sum
}

pub fn problem_2(data: &str) -> i64 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let regions = build_regions(&grid);
    let mut sum = 0_i64;
    for region in &regions {
        let mut perimeter = 0_i64;
        let mut area = 0_i64;
        for (r, c) in region.iter().copied() {
            area += 1;
            perimeter += 4;
            let plant = grid[r][c];
            for (r, c) in Neighbors::new(&grid, r, c) {
                if grid[r][c] == plant {
                    perimeter -= 1;
                }
            }
        }

        // Now we deduct shared continuous borders

        // First we check if (r,c) shares a continuous verical border with (r+1,c)
        // on either side for each (r,c) in the region.
        for (r, c) in region.iter().copied() {
            // If we're on the bottom of the grid, skip becuase there is no square
            // below to share a border with.
            if r + 1 == grid.len() {
                continue;
            }
            // If the box below is different, skip
            if grid[r + 1][c] != grid[r][c] {
                continue;
            }
            // First check the left hand side. If there is a continuous vertical
            // border then the two squares to the left have a different plant
            // (or no plant if on the grid edge).
            let different_left = match c.checked_sub(1) {
                None => true,
                Some(c_minus_1) => {
                    grid[r][c_minus_1] != grid[r][c] && grid[r + 1][c_minus_1] != grid[r + 1][c]
                }
            };
            if different_left {
                perimeter -= 1;
            }
            // Same for the right
            let different_right = if c + 1 == grid[0].len() {
                true
            } else {
                grid[r][c + 1] != grid[r][c] && grid[r + 1][c + 1] != grid[r + 1][c]
            };
            if different_right {
                perimeter -= 1;
            }
        }

        // Next (r,c) and (r+1,c)
        for (r, c) in region.iter().copied() {
            // If we're on the right side of grid, skip becuase there is no square
            // right to share a border with.
            if c + 1 == grid.len() {
                continue;
            }
            // If the box to the right is different, skip
            if grid[r][c + 1] != grid[r][c] {
                continue;
            }
            // First check above. If there is a continuous vertical
            // border then the two squares above have a different plant
            // (or no plant if on the grid edge).
            let different_above = match r.checked_sub(1) {
                None => true,
                Some(r_minus_1) => {
                    grid[r_minus_1][c] != grid[r][c] && grid[r_minus_1][c + 1] != grid[r][c + 1]
                }
            };
            if different_above {
                perimeter -= 1;
            }
            // Same for the bottom
            let different_bottom = if r + 1 == grid.len() {
                true
            } else {
                grid[r + 1][c] != grid[r][c] && grid[r + 1][c + 1] != grid[r][c + 1]
            };
            if different_bottom {
                perimeter -= 1;
            }
        }

        sum += area * perimeter;
    }
    sum
}

fn build_regions(grid: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut regions = HashMap::<(usize, usize), Vec<(usize, usize)>>::new();
    let mut visited: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();
    let mut to_visit: Vec<(usize, usize)> = vec![];
    for (r, row) in grid.iter().enumerate() {
        for (c, &plant) in row.iter().enumerate() {
            if visited[r][c] {
                continue;
            }
            // We are going to visit every square in the area containing this square
            to_visit.push((r, c));
            visited[r][c] = true;
            let regions = regions.entry((r, c)).or_default();
            while let Some((r, c)) = to_visit.pop() {
                for (r, c) in Neighbors::new(grid, r, c) {
                    if visited[r][c] {
                        continue;
                    }
                    if grid[r][c] != plant {
                        continue;
                    }
                    to_visit.push((r, c));
                    visited[r][c] = true;
                }
                regions.push((r, c));
            }
        }
    }
    regions.into_values().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const DATA_2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const DATA_3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const DATA_4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_problem_1_data_1() {
        assert_eq!(140, problem_1(DATA_1));
    }

    #[test]
    fn test_problem_1_data_2() {
        assert_eq!(1930, problem_1(DATA_2));
    }

    #[test]
    fn test_problem_2_data_1() {
        assert_eq!(80, problem_2(DATA_1));
    }

    #[test]
    fn test_problem_2_data_2() {
        assert_eq!(1206, problem_2(DATA_2));
    }

    #[test]
    fn test_problem_2_data_3() {
        assert_eq!(236, problem_2(DATA_3));
    }

    #[test]
    fn test_problem_2_data_4() {
        assert_eq!(368, problem_2(DATA_4));
    }
}
