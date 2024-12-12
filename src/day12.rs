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

pub fn problem_2(_data: &str) -> i64 {
    0
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

    #[test]
    fn test_problem_1_data_1() {
        assert_eq!(140, problem_1(DATA_1));
    }

    #[test]
    fn test_problem_1_data_2() {
        assert_eq!(1930, problem_1(DATA_2));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(0, problem_2(DATA_1));
    }
}
