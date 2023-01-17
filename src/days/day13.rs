use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
enum PacketValue {
    Integer(u32),
    List(Vec<PacketValue>),
}

impl PacketValue {
    fn is_list(&self) -> bool {
        matches!(self, PacketValue::List(_))
    }

    fn is_int(&self) -> bool {
        matches!(self, PacketValue::Integer(_))
    }

    fn convert_to_list(self) -> PacketValue {
        if self.is_list() {
            self
        } else {
            PacketValue::List(vec![self])
        }
    }
}

impl PartialOrd<Self> for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_list() && other.is_list() {
            if let PacketValue::List(l1) = self {
                if let PacketValue::List(l2) = other {
                    for (p1, p2) in l1.iter().zip(l2.iter()) {
                        if let Some(ord) = p1.partial_cmp(p2) {
                            if ord == Ordering::Greater {
                                return Some(Ordering::Greater);
                            } else if ord == Ordering::Less {
                                return Some(Ordering::Less);
                            }
                        }
                    }
                    return if l1.len() < l2.len() {
                        Some(Ordering::Less)
                    } else if l1.len() > l2.len() {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Equal)
                    };
                }
            }
        } else if self.is_list() && other.is_int() || self.is_int() && other.is_list() {
            let l1 = self.clone().convert_to_list();
            let l2 = other.clone().convert_to_list();
            return l1.partial_cmp(&l2);
        } else if let PacketValue::Integer(i1) = self {
            if let PacketValue::Integer(i2) = other {
                return i1.partial_cmp(i2);
            }
        }

        None
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for PacketValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        let mut i = 0;
        while i < s.len() {
            if s.as_bytes()[i] == b'[' {
                stack.push(Vec::new());
                i += 1;
            } else if s.as_bytes()[i] == b']' && i != s.len() - 1 {
                let last = stack.pop().unwrap();
                stack.last_mut().unwrap().push(PacketValue::List(last));
                i += 1;
            } else if s.as_bytes()[i] != b']' && s.as_bytes()[i] != b',' {
                let end_pos = s[i..]
                    .as_bytes()
                    .iter()
                    .position(|&b| b == b',' || b == b']')
                    .unwrap();
                stack.last_mut().unwrap().push(PacketValue::Integer(
                    u32::from_str(&s[i..(i + end_pos)]).unwrap(),
                ));
                i += end_pos;
            } else {
                i += 1;
            }
        }

        Ok(PacketValue::List(stack.pop().unwrap()))
    }
}

pub fn part1(input: &[&str]) -> u32 {
    let mut sum = 0;

    input.chunks(3).enumerate().for_each(|(i, packets)| {
        let packet1 = PacketValue::from_str(packets[0]).unwrap();
        let packet2 = PacketValue::from_str(packets[1]).unwrap();

        if let Some(ord) = packet1.partial_cmp(&packet2) {
            if ord == Ordering::Less {
                sum += i + 1;
            }
        }
    });

    sum as u32
}

pub fn part2(input: &[&str]) -> u32 {
    let mut all_packets = Vec::with_capacity((input.len() / 3) * 2);
    input.chunks(3).enumerate().for_each(|(_, packets)| {
        let packet1 = PacketValue::from_str(packets[0]).unwrap();
        let packet2 = PacketValue::from_str(packets[1]).unwrap();

        all_packets.push(packet1);
        all_packets.push(packet2);
    });

    all_packets.push(PacketValue::List(vec![PacketValue::List(vec![
        PacketValue::Integer(2),
    ])]));
    all_packets.push(PacketValue::List(vec![PacketValue::List(vec![
        PacketValue::Integer(6),
    ])]));

    all_packets.sort();

    all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p == &PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(2)])])
                || p == &PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(6)])])
            {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>() as u32
}
