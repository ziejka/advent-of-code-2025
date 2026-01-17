use crate::Part;

pub fn run(part: crate::Part) {
    match part {
        Part::A => part_a("src/day04/input.txt"),
        Part::B => part_b("src/day04/input.txt"),
        Part::T => part_b("src/day04/test.txt"),
    }
}

fn part_a(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");
    let mut size: usize = 0;

    let map: Vec<String> = file
        .lines()
        .flat_map(|line| {
            if size == 0 {
                size = line.len();
            }
            line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    let (_, count_isolated_elements) = find_and_mark_isolated_elements(&map, size);

    // print_map(&map_print, size);
    dbg!(count_isolated_elements);
}

fn find_and_mark_isolated_elements(map: &Vec<String>, size: usize) -> (Vec<String>, usize) {
    let mut debug_map: Vec<String> = vec![];
    let mut count_isolated = 0;
    let l = map.len() / size;
    for y in 0..l {
        for x in 0..size {
            let idx = y * size + x;
            let el = map.get(idx);
            if el.is_none() {
                continue;
            }
            if el.unwrap() != "@" {
                debug_map.push(".".to_string());
                continue;
            }
            let is_isolated = has_fewer_than_four_neighbors(&map, x as i32, y as i32, size as i32);
            if is_isolated {
                count_isolated += 1;
                debug_map.push("x".to_string());
                continue;
            }
            debug_map.push("@".to_string());
        }
    }

    (debug_map, count_isolated)
}

fn has_fewer_than_four_neighbors(map: &Vec<String>, x: i32, y: i32, size: i32) -> bool {
    let mut neighbor_count = 0;
    for i in -1i32..=1i32 {
        for j in -1i32..=1i32 {
            if i == 0 && j == 0 {
                continue;
            }
            if y + i < 0 || x + j < 0 || x + j >= size {
                continue;
            }
            let idx = (y + i) * size + x + j;
            let el = map.get(idx as usize);
            if el.is_none() {
                continue;
            }
            if el.unwrap() == "@" {
                neighbor_count += 1;
                if neighbor_count == 4 {
                    return false;
                }
            }
        }
    }
    true
}

fn print_map(map: &Vec<String>, size: usize) {
    let mut in_line = 0;
    map.iter().for_each(|x| {
        in_line += 1;
        print!("{}", x);
        if in_line >= size {
            println!("");
            in_line = 0;
        }
    });
}

fn part_b(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");
    let mut size: usize = 0;

    let map: Vec<String> = file
        .lines()
        .flat_map(|line| {
            if size == 0 {
                size = line.len();
            }
            line.chars().map(|c| c.to_string()).collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();
    let n = simulate_removal_steps(map, size, 0, 1);
    dbg!(n);
}

fn simulate_removal_steps(
    map: Vec<String>,
    size: usize,
    total_removed: usize,
    last_step_removed: usize,
) -> usize {
    if last_step_removed == 0 {
        return total_removed;
    }
    let (new_map, removed_in_step) = find_and_mark_isolated_elements(&map, size);
    simulate_removal_steps(
        new_map,
        size,
        total_removed + removed_in_step,
        removed_in_step,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_sides_no_neighbors() {
        let map: Vec<String> = vec![".", ".", ".", ".", "@", ".", ".", ".", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(has_fewer_than_four_neighbors(&map, 1, 1, 3), true);
    }

    #[test]
    fn test_check_sides_three_neighbors() {
        let map: Vec<String> = vec!["@", "@", "@", ".", "@", ".", ".", ".", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(has_fewer_than_four_neighbors(&map, 1, 1, 3), true);
    }

    #[test]
    fn test_check_sides_four_neighbors() {
        let map: Vec<String> = vec!["@", "@", ".", "@", "@", "@", ".", ".", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(has_fewer_than_four_neighbors(&map, 1, 1, 3), false);
    }

    #[test]
    fn test_check_sides_edge_position() {
        let map: Vec<String> = vec!["@", ".", ".", ".", ".", ".", ".", ".", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(has_fewer_than_four_neighbors(&map, 0, 0, 3), true);
    }

    #[test]
    fn test_check_sides_corner_with_many() {
        let map: Vec<String> = vec!["@", "@", ".", "@", "@", ".", ".", ".", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(has_fewer_than_four_neighbors(&map, 0, 0, 3), false);
    }
}
