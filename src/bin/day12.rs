use std::collections::VecDeque;
use std::convert::From;
use std::ops::Add;

fn gcd_euclid(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_euclid(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd_euclid(a, b)
}

#[derive(Debug)]
enum E {
    InvalidInputLine,
    InvalidNumber,
    WrongNumOfMoons,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
struct Vec3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3D {
    fn norm_l1(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

struct Moon {
    pos: Vec3D,
    vel: Vec3D,
}

impl From<(i32, i32, i32)> for Moon {
    fn from(pos: (i32, i32, i32)) -> Self {
        Moon {
            pos: Vec3D {
                x: pos.0,
                y: pos.1,
                z: pos.2,
            },
            vel: Vec3D::default(),
        }
    }
}

impl Moon {
    fn compute_gravity_1d(p_moon1: i32, p_moon2: i32) -> i32 {
        if p_moon2 > p_moon1 {
            1
        } else if p_moon2 < p_moon1 {
            -1
        } else {
            0
        }
    }

    fn apply_gravity(&mut self, other_moons: &VecDeque<Moon>) {
        for other in other_moons {
            self.vel.x += Moon::compute_gravity_1d(self.pos.x, other.pos.x);
            self.vel.y += Moon::compute_gravity_1d(self.pos.y, other.pos.y);
            self.vel.z += Moon::compute_gravity_1d(self.pos.z, other.pos.z);
        }
    }

    fn apply_velocity(&mut self) {
        self.pos = self.pos + self.vel;
    }

    fn get_energy(&self) -> i32 {
        self.pos.norm_l1() * self.vel.norm_l1()
    }
}

fn line_to_vec(line: String) -> aoc19::Result<Moon> {
    let line = line.trim_matches(|c| c == '<' || c == '>');
    let coords: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

    if coords.len() != 3 {
        return Err(aoc19::Error::boxed(E::InvalidInputLine));
    }

    let extract = |assignment: &str| {
        assignment
            .split('=')
            .nth(1)
            .ok_or(aoc19::Error::boxed(E::InvalidInputLine))
            .and_then(|num| {
                num.parse::<i32>()
                    .or(Err(aoc19::Error::boxed(E::InvalidNumber)))
            })
    };

    Ok(Moon::from((
        extract(coords[0])?,
        extract(coords[1])?,
        extract(coords[2])?,
    )))
}

fn read_input() -> aoc19::Result<VecDeque<Moon>> {
    let input = std::fs::read_to_string("input/day12")?;

    let mut moons = VecDeque::new();
    for line in input.lines() {
        moons.push_back(line_to_vec(line.to_string())?);
    }
    Ok(moons)
}

fn part_one() -> aoc19::Result<i32> {
    let mut moons = read_input()?;
    if moons.len() != 4 {
        return Err(aoc19::Error::boxed(E::WrongNumOfMoons));
    }

    for _ in 0..1000 {
        for _ in 0..moons.len() {
            let mut moon = moons.pop_front().unwrap();
            moon.apply_gravity(&moons);
            moons.push_back(moon);
        }
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    let total_energy = moons.iter().fold(0, |e, m| e + m.get_energy());
    Ok(total_energy)
}

fn part_two() -> aoc19::Result<u64> {
    let moons = read_input()?;

    let mut moon_dims = vec![VecDeque::new(); 3];
    for moon in moons {
        moon_dims[0].push_back((moon.pos.x, moon.vel.x));
        moon_dims[1].push_back((moon.pos.y, moon.vel.y));
        moon_dims[2].push_back((moon.pos.z, moon.vel.z));
    }

    let iterations_1d: Vec<_> = moon_dims
        .iter_mut()
        .map(|moons_1d: &mut VecDeque<_>| {
            let init_state = moons_1d.to_owned();

            let mut cnt: u64 = 0;
            loop {
                for _ in 0..moons_1d.len() {
                    let (p, mut v) = moons_1d.pop_front().unwrap();
                    // apply gravity
                    for (other_p, _) in moons_1d.iter() {
                        v += Moon::compute_gravity_1d(p, *other_p);
                    }
                    moons_1d.push_back((p, v));
                }
                // apply velocity
                for (ref mut p, v) in moons_1d.iter_mut() {
                    *p += *v;
                }

                cnt += 1;
                if *moons_1d == init_state {
                    break cnt;
                }
            }
        })
        .collect();

    Ok(iterations_1d[1..]
        .iter()
        .fold(iterations_1d[0], |last, curr| lcm(last, *curr)))
}

fn main() -> aoc19::Result<()> {
    println!(
        "Part One: Total energy after 1000 time steps {}",
        part_one()?
    );
    println!("Part Two: Number of steps {}", part_two()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one().unwrap(), 9139)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two().unwrap(), 420788524631496)
    }
}
