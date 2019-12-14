use aoc19::Result;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Vec2D(i32, i32);
type Asteroids = Vec<Vec2D>;

impl std::ops::Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2D(self.0 - other.0, self.1 - other.1)
    }
}

fn read_input() -> aoc19::Result<String> {
    std::fs::read_to_string("input/day10").map_err(|e| e.into())
}

fn create_asteroids(input: &String) -> aoc19::Result<Asteroids> {
    let mut belt = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, elem) in line.chars().enumerate() {
            if elem == '#' {
                belt.push(Vec2D(i32::try_from(x)?, i32::try_from(y)?));
            }
        }
    }
    Ok(belt)
}

fn gcd_euclid(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd_euclid(b, a % b)
    }
}

fn normalize(v: &Vec2D) -> Vec2D {
    let gcd = gcd_euclid(v.0, v.1).abs();
    Vec2D(v.0 / gcd, v.1 / gcd)
}

fn compute_visible(origin: &Vec2D, asteroids: &Asteroids) -> usize {
    let normed_dist = asteroids
        .iter()
        .filter(|Vec2D(x, y)| !(origin.0 == *x && origin.1 == *y))
        .map(|asteroid| normalize(&(*asteroid - *origin)))
        .collect::<HashSet<Vec2D>>();
    normed_dist.len()
}

fn part_one() -> aoc19::Result<usize> {
    let asteroids = create_asteroids(&read_input()?)?;

    let max = asteroids
        .iter()
        .map(|asteroid| compute_visible(asteroid, &asteroids))
        .fold(0, |val, max| std::cmp::max(val, max));

    Ok(max)
}

fn main() -> aoc19::Result<()> {
    println!("Part one: max num visible asteroids {}", part_one()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        // .7..7
        // .....
        // 67775
        // ....7
        // ...87
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##"
            .to_string();

        let asteroids = create_asteroids(&input).unwrap();
        assert_eq!(compute_visible(&Vec2D(1, 0), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(4, 0), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(0, 2), &asteroids), 6);
        assert_eq!(compute_visible(&Vec2D(1, 2), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(2, 2), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(3, 2), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(4, 2), &asteroids), 5);
        assert_eq!(compute_visible(&Vec2D(4, 3), &asteroids), 7);
        assert_eq!(compute_visible(&Vec2D(3, 4), &asteroids), 8);
        assert_eq!(compute_visible(&Vec2D(4, 4), &asteroids), 7);
    }

    #[test]
    fn test_example2() {
        let input = "......#.#.\n\
                     #..#.#....\n\
                     ..#######.\n\
                     .#.#.###..\n\
                     .#..#.....\n\
                     ..#....#.#\n\
                     #..#....#.\n\
                     .##.#..###\n\
                     ##...#..#.\n\
                     .#....####"
            .to_string();

        let asteroids = create_asteroids(&input).unwrap();
        assert_eq!(compute_visible(&Vec2D(5, 8), &asteroids), 33);
    }

    #[test]
    fn test_example3() {
        let input = "#.#...#.#.\n\
                     .###....#.\n\
                     .#....#...\n\
                     ##.#.#.#.#\n\
                     ....#.#.#.\n\
                     .##..###.#\n\
                     ..#...##..\n\
                     ..##....##\n\
                     ......#...\n\
                     .####.###."
            .to_string();

        let asteroids = create_asteroids(&input).unwrap();
        assert_eq!(compute_visible(&Vec2D(1, 2), &asteroids), 35);
    }

    #[test]
    fn test_example4() {
        let input = ".#..#..###\n\
                     ####.###.#\n\
                     ....###.#.\n\
                     ..###.##.#\n\
                     ##.##.#.#.\n\
                     ....###..#\n\
                     ..#.#..#.#\n\
                     #..#.#.###\n\
                     .##...##.#\n\
                     .....#.#.."
            .to_string();

        let asteroids = create_asteroids(&input).unwrap();
        assert_eq!(compute_visible(&Vec2D(6, 3), &asteroids), 41);
    }

    #[test]
    fn test_example5() {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##"
            .to_string();

        let asteroids = create_asteroids(&input).unwrap();
        assert_eq!(compute_visible(&Vec2D(11, 13), &asteroids), 210);
    }
}
