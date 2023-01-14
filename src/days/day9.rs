use std::collections::BTreeSet;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct RopeKnot {
    pos: (i32, i32),
}

impl RopeKnot {
    fn new(x: i32, y: i32) -> Self {
        Self { pos: (x, y) }
    }

    fn distance_from(&self, rhs: &Self) -> [Displacement; 2] {
        let mut index = 0;
        let mut displacements = [Displacement::None; 2];
        let x1 = self.pos.0;
        let x2 = rhs.pos.0;
        let y1 = self.pos.1;
        let y2 = rhs.pos.1;
        if x1 < x2 {
            displacements[index] = Displacement::Left(x2 - x1);
            index += 1;
        }
        if x2 < x1 {
            displacements[index] = Displacement::Right(x1 - x2);
            index += 1;
        }
        if y1 < y2 {
            displacements[index] = Displacement::Up(y2 - y1);
        }
        if y2 < y1 {
            displacements[index] = Displacement::Down(y1 - y2);
        }
        displacements
    }

    fn displace(&mut self, displacement: Displacement) {
        match displacement {
            Displacement::Up(i) => self.pos.1 -= i,
            Displacement::Down(i) => self.pos.1 += i,
            Displacement::Right(i) => self.pos.0 += i,
            Displacement::Left(i) => self.pos.0 -= i,
            _ => {}
        }
    }

    fn follow(&mut self, rhs: &Self) {
        self.pos.0 = self.pos.0 + (rhs.pos.0 - self.pos.0).signum();
        self.pos.1 = self.pos.1 + (rhs.pos.1 - self.pos.1).signum();
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Displacement {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
    None,
}

impl Displacement {
    fn unwrap(self) -> i32 {
        match self {
            Displacement::Up(i) => i,
            Displacement::Down(i) => i,
            Displacement::Right(i) => i,
            Displacement::Left(i) => i,
            Displacement::None => 0,
        }
    }

    fn unit(self) -> Self {
        match self {
            Displacement::Up(_) => Displacement::Up(1),
            Displacement::Down(_) => Displacement::Down(1),
            Displacement::Right(_) => Displacement::Right(1),
            Displacement::Left(_) => Displacement::Left(1),
            Displacement::None => Displacement::None,
        }
    }
}

impl Sub<i32> for Displacement {
    type Output = Displacement;

    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Displacement::Up(i) => Displacement::Up(i - rhs),
            Displacement::Down(i) => Displacement::Down(i - rhs),
            Displacement::Right(i) => Displacement::Right(i - rhs),
            Displacement::Left(i) => Displacement::Left(i - rhs),
            Displacement::None => Displacement::None,
        }
    }
}

impl FromStr for Displacement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(' ').ok_or(())?;
        let amount = i32::from_str(amount).map_err(|_| ())?;
        let dis = match dir {
            "D" => Displacement::Down(amount),
            "R" => Displacement::Right(amount),
            "U" => Displacement::Up(amount),
            "L" => Displacement::Left(amount),
            _ => return Err(()),
        };
        Ok(dis)
    }
}

pub fn part1(input: &[&str]) -> usize {
    let mut tail_pos = RopeKnot::new(0, 0);
    let mut head_pos = RopeKnot::new(0, 0);
    let mut visited = BTreeSet::new();
    visited.insert(tail_pos);
    for line in input {
        head_pos.displace(Displacement::from_str(line).unwrap());
        let displacements = head_pos.distance_from(&tail_pos);
        if displacements.iter().all(|&d| d.unwrap() <= 1) {
            // Tail and Head are touching, no need to move the tail
            continue;
        }
        let smaller_distance_pos = if displacements[0].unwrap() < displacements[1].unwrap() {
            0
        } else {
            1
        };

        tail_pos.displace(displacements[smaller_distance_pos]);
        move_knot_and_log(
            &mut tail_pos,
            &mut visited,
            displacements[1 - smaller_distance_pos],
            true,
        );
    }

    visited.len()
}

fn move_knot_and_log(
    knot_pos: &mut RopeKnot,
    visited: &mut BTreeSet<RopeKnot>,
    displacement: Displacement,
    log: bool,
) {
    match displacement {
        Displacement::Up(i) => {
            for _ in 1..i {
                knot_pos.displace(Displacement::Up(1));
                if log {
                    visited.insert(*knot_pos);
                }
            }
        }
        Displacement::Down(i) => {
            for _ in 1..i {
                knot_pos.displace(Displacement::Down(1));
                if log {
                    visited.insert(*knot_pos);
                }
            }
        }
        Displacement::Right(i) => {
            for _ in 1..i {
                knot_pos.displace(Displacement::Right(1));
                if log {
                    visited.insert(*knot_pos);
                }
            }
        }
        Displacement::Left(i) => {
            for _ in 1..i {
                knot_pos.displace(Displacement::Left(1));
                if log {
                    visited.insert(*knot_pos);
                }
            }
        }
        _ => {}
    }
}

pub fn part2(input: &[&str]) -> usize {
    let mut knots = [RopeKnot::new(0, 0); 10];
    let mut visited = BTreeSet::new();
    visited.insert(knots[9]);
    for line in input {
        let displacement = Displacement::from_str(line).unwrap();

        for _ in 0..displacement.unwrap() {
            let unit_disp = displacement.unit();
            knots[0].displace(unit_disp);
            for i in 1..10 {
                let (head, mut tail) = (knots[i - 1], knots[i]);
                let displacements = head.distance_from(&tail);
                if displacements.iter().all(|&d| d.unwrap() <= 1) {
                    // Tail and Head are touching, no need to move the tail
                    continue;
                }
                tail.follow(&head);
                knots[i] = tail;
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use crate::{day9, parse_input_static};

    #[test]
    fn test_part1() {
        let input = parse_input_static(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );

        assert_eq!(13, day9::part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = parse_input_static(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );

        assert_eq!(36, day9::part2(&input));
    }
}
