use crate::Part;

pub fn run(part: crate::Part) {
    match part {
        Part::A => part_a("src/day02/input.txt"),
        Part::B => part_b("src/day02/input.txt"),
        Part::T => part_b("src/day02/test.txt"),
    }
}

fn part_a(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");

    let r: u64 = file
        .split(',')
        .flat_map(|pair| {
            let (a, b) = pair.trim().split_once('-').unwrap();
            let min = a.parse::<u64>().expect(&format!("Can't pare a {}", a));
            let max = b.parse::<u64>().expect(&format!("Can't pare b {}", b));
            min..=max
        })
        .filter(|n| is_repeated(&n.to_string()))
        .sum();

    dbg!(r);
}

fn is_repeated(s: &str) -> bool {
    let (l, r) = s.split_at(s.len() / 2);
    l == r
}

fn part_b(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");

    let r: u64 = file
        .split(',')
        .flat_map(|pair| {
            let (a, b) = pair.trim().split_once('-').unwrap();
            let min = a.parse::<u64>().expect(&format!("Can't pare a {}", a));
            let max = b.parse::<u64>().expect(&format!("Can't pare b {}", b));
            min..=max
        })
        .filter(|n| is_repeated_extended(&n.to_string()))
        .sum();

    dbg!(r);
}

fn is_repeated_extended(s: &str) -> bool {
    if is_repeated(s) {
        return true;
    }
    for i in 1..s.len() / 2 + 1 {
        let mut chunks = s.as_bytes().chunks(i);
        let first = chunks.next().unwrap();
        if chunks.clone().all(|chunk| chunk == first) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_repeated() {
        assert_eq!(is_repeated("123123"), true);
        assert_eq!(is_repeated("11"), true);
        assert_eq!(is_repeated("12"), false);
    }
}
