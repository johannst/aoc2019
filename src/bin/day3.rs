use std::collections::HashSet;
use std::iter::FromIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Pos2D = (i32, i32);

#[derive(Copy,Clone)]
enum Action {
    U(i32),
    D(i32),
    R(i32),
    L(i32),
}

fn unroll_action(c: &mut Vec<Pos2D>, a: Action, mut p: Pos2D) -> Pos2D {
    let step: fn(x: Pos2D) -> Pos2D;
    let num = match a {
        Action::U(s) => {
            step = |x| (x.0, x.1 + 1);
            s
        }
        Action::D(s) => {
            step = |x| (x.0, x.1 - 1);
            s
        }
        Action::R(s) => {
            step = |x| (x.0 + 1, x.1);
            s
        }
        Action::L(s) => {
            step = |x| (x.0 - 1, x.1);
            s
        }
    };

    c.reserve(num as usize);
    for _ in 0..num {
        p = step(p);
        c.push(p);
    }
    p
}

fn compute_wire(actions: &Vec<Action>) -> Vec<Pos2D> {
    let mut pos: Pos2D = (0, 0);
    let mut coords: Vec<Pos2D> = Vec::new();

    for action in actions {
        pos = unroll_action(&mut coords, *action, pos);
    }
    coords
}

fn compute_manhattan_distance(wire1: &Vec<Pos2D>, wire2: &Vec<Pos2D>) -> i32 {
    let make_set = |vec: &Vec<Pos2D>| -> HashSet<Pos2D> { HashSet::from_iter(vec.iter().cloned()) };
    make_set(&wire1)
        .intersection(&make_set(&wire2))
        .fold(std::i32::MAX, |dist, coord| {
            std::cmp::min(dist, coord.0.abs() + coord.1.abs())
        })
}

fn compute_fewest_steps(wire1: &Vec<Pos2D>, wire2: &Vec<Pos2D>) -> i32 {
    let make_set = |vec: &Vec<Pos2D>| -> HashSet<Pos2D> { HashSet::from_iter(vec.iter().cloned()) };
    make_set(&wire1)
        .intersection(&make_set(&wire2))
        .fold(std::i32::MAX, |steps, inter| {
            std::cmp::min(
                steps,
                wire1.iter().position(|x| x == inter).unwrap() as i32 + 1 +
                wire2.iter().position(|x| x == inter).unwrap() as i32 + 1
            )
        })
}

fn main() -> Result<()> {
    let wires = {
        let fname = std::env::args().nth(1).unwrap_or_else(|| {
            println!("usage: d03 <file>");
            std::process::exit(1);
        });

        let actionize = |input: &str| -> Result<Action> {
            let mut input = input.chars();
            let a = input.next();
            let steps = input.as_str().parse::<i32>()?;
            let res = match a {
                Some(a) if a == 'U' => Action::U(steps),
                Some(a) if a == 'D' => Action::D(steps),
                Some(a) if a == 'R' => Action::R(steps),
                Some(a) if a == 'L' => Action::L(steps),
                _ => unimplemented!(),
            };
            Ok(res)
        };

        let wire_descriptions = std::fs::read_to_string(fname)?;
        let mut wires: Vec<Vec<Action>> = Vec::new();
        for wire_description in wire_descriptions.lines() {
            let wire = wire_description
                .split(',')
                .map(|action| actionize(action))
                .collect::<Result<Vec<Action>>>()?;
            wires.push(wire);
        }

        wires
    };

    if wires.len() != 2 {
        println!(
            "Error: Input contained {} wires, expected only two wires!",
            wires.len()
        );
        std::process::exit(1);
    }

    println!(
        "Part One: manhattan distance = {}",
        compute_manhattan_distance(&compute_wire(&wires[0]), &compute_wire(&wires[1]))
    );

    println!(
        "Part Two: intersection with fewest steps = {} steps",
        compute_fewest_steps(&compute_wire(&wires[0]), &compute_wire(&wires[1]))
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Action::*;
    use super::*;

    #[test]
    fn test_example1() {
        let w1 = vec![R(8), U(5), L(5), D(3)];
        let w2 = vec![U(7), R(6), D(4), L(4)];

        assert_eq!(
            compute_manhattan_distance(&compute_wire(&w1), &compute_wire(&w2)),
            6
        );
    }

    #[test]
    fn test_example2() {
        let w1 = vec![R(75), D(30), R(83), U(83), L(12), D(49), R(71), U(7), L(72)];
        let w2 = vec![U(62), R(66), U(55), R(34), D(71), R(55), D(58), R(83)];

        assert_eq!(
            compute_manhattan_distance(&compute_wire(&w1), &compute_wire(&w2)),
            159
        );
    }

    #[test]
    fn test_example3() {
        let w1 = vec![
            R(98),
            U(47),
            R(26),
            D(63),
            R(33),
            U(87),
            L(62),
            D(20),
            R(33),
            U(53),
            R(51),
        ];
        let w2 = vec![
            U(98),
            R(91),
            D(20),
            R(16),
            D(67),
            R(40),
            U(7),
            R(15),
            U(6),
            R(7),
        ];

        assert_eq!(
            compute_manhattan_distance(&compute_wire(&w1), &compute_wire(&w2)),
            135
        );
    }

    #[test]
    fn test2_example1() {
        let w1 = vec![R(8), U(5), L(5), D(3)];
        let w2 = vec![U(7), R(6), D(4), L(4)];

        assert_eq!(
            compute_fewest_steps(&compute_wire(&w1), &compute_wire(&w2)),
            30
        );
    }

    #[test]
    fn test2_example2() {
        let w1 = vec![R(75), D(30), R(83), U(83), L(12), D(49), R(71), U(7), L(72)];
        let w2 = vec![U(62), R(66), U(55), R(34), D(71), R(55), D(58), R(83)];

        assert_eq!(
            compute_fewest_steps(&compute_wire(&w1), &compute_wire(&w2)),
            610
        );
    }

    #[test]
    fn test2_example3() {
        let w1 = vec![
            R(98),
            U(47),
            R(26),
            D(63),
            R(33),
            U(87),
            L(62),
            D(20),
            R(33),
            U(53),
            R(51),
        ];
        let w2 = vec![
            U(98),
            R(91),
            D(20),
            R(16),
            D(67),
            R(40),
            U(7),
            R(15),
            U(6),
            R(7),
        ];

        assert_eq!(
            compute_fewest_steps(&compute_wire(&w1), &compute_wire(&w2)),
            410
        );
    }
}
