use crate::Part;

pub fn run(part: crate::Part) {
    match part {
        Part::A => part_a("src/day03/input.txt"),
        Part::B => part_b("src/day03/input.txt"),
        Part::T => part_a("src/day03/test.txt"),
    }
}

fn part_a(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");

    let r: u32 = file
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut h = 0;
            for i in 0..numbers.len() - 1 {
                for j in i + 1..numbers.len() {
                    if h < numbers[i] * 10 + numbers[j] {
                        h = numbers[i] * 10 + numbers[j];
                    }
                }
            }
            h
        })
        .sum();

    dbg!(r);
}

fn part_b(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");

    let r: u32 = file
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut h = 0;
            for i in 0..numbers.len() - 1 {
                for j in i + 1..numbers.len() {
                    if h < numbers[i] * 10 + numbers[j] {
                        h = numbers[i] * 10 + numbers[j];
                    }
                }
            }
            h
        })
        .sum();

    dbg!(r);
}

// Get max of first digit 0..n-12, then max of index_of_previous+1..n-12-number_of_selected_digit
fn _step(size: usize, numbers: Vec<String>, curr: String) {
    if size == 0 {
        return;
    }

    let _result = curr + numbers.first().unwrap();
    // numbers.

    // return step(size - 1, numbers, result);
}
