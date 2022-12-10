use std::collections::BTreeMap;
use std::collections::VecDeque;

pub fn part_1() -> String {
    let mut stacks = get_stacks();
    let commands = get_commands();

    for command in commands {
        for _ in 0..command.count {
            let from_crate = stacks.get_mut(&command.from).unwrap();
            let moved_crate = from_crate.pop_front().unwrap();

            stacks.get_mut(&command.to).unwrap().push_front(moved_crate);
        }
    }

    let result = stacks.values().map(|crates| crates.get(0).unwrap()).collect::<String>();

    result
}

pub fn part_2() -> String {
    let mut stacks = get_stacks();
    let commands = get_commands();

    for command in commands {
        let from_crate = stacks.get_mut(&command.from).unwrap();
        let moved_crates = from_crate.drain(..command.count as usize).collect::<Vec<char>>();

        for moved_crate in moved_crates.iter().rev() {
            stacks.get_mut(&command.to).unwrap().push_front(*moved_crate);
        }
    }

    let result = stacks.values().map(|crates| crates.get(0).unwrap()).collect::<String>();

    result
}

fn get_stacks() -> BTreeMap<u32, VecDeque<char>> {
    BTreeMap::from([
        (1, "TZB".chars().collect::<VecDeque<char>>()),
        (2, "NDTHV".chars().collect::<VecDeque<char>>()),
        (3, "DMFB".chars().collect::<VecDeque<char>>()),
        (4, "LQVWGJT".chars().collect::<VecDeque<char>>()),
        (5, "MQFVPGDW".chars().collect::<VecDeque<char>>()),
        (6, "SFHGQZV".chars().collect::<VecDeque<char>>()),
        (7, "WCTLRNSZ".chars().collect::<VecDeque<char>>()),
        (8, "MRNJDWHZ".chars().collect::<VecDeque<char>>()),
        (9, "SDFLQM".chars().collect::<VecDeque<char>>()),
    ])
}

fn get_commands() -> Vec<Command> {
    let input: Vec<&str> = INPUT.split("\n\n").collect();
    let commands = input[1];

    commands.lines().map(|line| {
        let numbers: Vec<u32> = line
            .replace("move ", "")
            .replace("from", "")
            .replace("to", "")
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Command::new(numbers[0], numbers[1], numbers[2])
    }).collect()
}

struct Command {
    pub count: u32,
    pub from: u32,
    pub to: u32
}

impl Command {
    pub fn new(count: u32, from: u32, to: u32) -> Self {
        Self { count, from, to }
    }
}

static INPUT: &str = "                [M]     [W] [M]
            [L] [Q] [S] [C] [R]
            [Q] [F] [F] [T] [N] [S]
    [N]     [V] [V] [H] [L] [J] [D]
    [D] [D] [W] [P] [G] [R] [D] [F]
[T] [T] [M] [G] [G] [Q] [N] [W] [L]
[Z] [H] [F] [J] [D] [Z] [S] [H] [Q]
[B] [V] [B] [T] [W] [V] [Z] [Z] [M]
 1   2   3   4   5   6   7   8   9

move 1 from 7 to 4
move 1 from 6 to 2
move 5 from 9 to 4
move 2 from 2 to 8
move 2 from 2 to 6
move 3 from 3 to 7
move 3 from 7 to 1
move 1 from 9 to 4
move 4 from 7 to 3
move 5 from 1 to 8
move 1 from 1 to 2
move 1 from 4 to 9
move 4 from 5 to 6
move 1 from 5 to 8
move 3 from 2 to 4
move 3 from 6 to 4
move 3 from 5 to 9
move 1 from 9 to 7
move 1 from 9 to 8
move 2 from 7 to 9
move 1 from 7 to 9
move 1 from 7 to 8
move 2 from 9 to 8
move 13 from 4 to 2
move 2 from 6 to 1
move 3 from 3 to 2
move 9 from 2 to 7
move 1 from 9 to 7
move 5 from 7 to 8
move 9 from 8 to 4
move 2 from 7 to 1
move 1 from 1 to 7
move 2 from 3 to 2
move 14 from 4 to 5
move 3 from 8 to 4
move 13 from 8 to 3
move 2 from 7 to 1
move 6 from 6 to 5
move 1 from 6 to 9
move 3 from 2 to 8
move 1 from 7 to 8
move 5 from 2 to 8
move 4 from 4 to 8
move 1 from 9 to 8
move 1 from 7 to 1
move 1 from 1 to 2
move 7 from 3 to 2
move 4 from 3 to 2
move 2 from 5 to 3
move 2 from 1 to 5
move 5 from 8 to 7
move 6 from 8 to 3
move 11 from 5 to 8
move 8 from 8 to 9
move 5 from 7 to 8
move 3 from 1 to 8
move 5 from 3 to 8
move 11 from 2 to 9
move 1 from 3 to 5
move 1 from 2 to 1
move 1 from 2 to 7
move 6 from 5 to 7
move 19 from 9 to 7
move 3 from 5 to 3
move 1 from 5 to 4
move 1 from 1 to 4
move 1 from 9 to 8
move 25 from 7 to 9
move 2 from 4 to 1
move 2 from 1 to 4
move 2 from 4 to 7
move 2 from 7 to 9
move 5 from 3 to 1
move 1 from 7 to 1
move 9 from 9 to 5
move 3 from 5 to 6
move 9 from 9 to 1
move 7 from 1 to 3
move 6 from 8 to 9
move 1 from 5 to 2
move 10 from 9 to 2
move 1 from 2 to 7
move 5 from 9 to 8
move 1 from 5 to 8
move 5 from 3 to 8
move 1 from 9 to 4
move 4 from 3 to 6
move 4 from 6 to 3
move 3 from 1 to 4
move 3 from 2 to 4
move 3 from 5 to 8
move 3 from 4 to 9
move 1 from 7 to 3
move 2 from 9 to 8
move 4 from 2 to 5
move 1 from 3 to 4
move 1 from 9 to 3
move 5 from 5 to 6
move 7 from 8 to 5
move 3 from 1 to 7
move 6 from 5 to 8
move 5 from 4 to 5
move 3 from 3 to 2
move 1 from 1 to 4
move 19 from 8 to 1
move 3 from 7 to 3
move 4 from 2 to 9
move 1 from 2 to 6
move 7 from 6 to 4
move 1 from 6 to 2
move 2 from 1 to 3
move 5 from 4 to 1
move 1 from 6 to 2
move 3 from 3 to 6
move 12 from 1 to 2
move 2 from 8 to 1
move 14 from 2 to 4
move 7 from 1 to 5
move 10 from 4 to 6
move 3 from 6 to 4
move 1 from 8 to 4
move 4 from 3 to 5
move 1 from 2 to 3
move 2 from 1 to 4
move 17 from 5 to 3
move 7 from 4 to 1
move 1 from 9 to 4
move 4 from 6 to 3
move 5 from 4 to 8
move 12 from 3 to 1
move 6 from 3 to 5
move 17 from 1 to 5
move 2 from 1 to 9
move 3 from 1 to 4
move 7 from 8 to 2
move 4 from 3 to 7
move 1 from 1 to 8
move 17 from 5 to 2
move 11 from 2 to 8
move 11 from 8 to 4
move 11 from 2 to 4
move 4 from 6 to 1
move 4 from 1 to 3
move 2 from 6 to 9
move 3 from 7 to 8
move 3 from 5 to 3
move 23 from 4 to 3
move 4 from 4 to 8
move 1 from 7 to 4
move 2 from 2 to 3
move 6 from 3 to 2
move 16 from 3 to 9
move 2 from 5 to 8
move 1 from 4 to 5
move 2 from 5 to 9
move 1 from 2 to 3
move 1 from 3 to 8
move 9 from 9 to 1
move 6 from 3 to 8
move 3 from 3 to 1
move 18 from 8 to 9
move 1 from 3 to 5
move 5 from 1 to 4
move 5 from 1 to 8
move 3 from 4 to 1
move 1 from 5 to 2
move 2 from 4 to 8
move 1 from 1 to 2
move 5 from 2 to 7
move 2 from 8 to 1
move 2 from 2 to 6
move 3 from 1 to 6
move 3 from 9 to 6
move 31 from 9 to 7
move 26 from 7 to 8
move 3 from 1 to 6
move 22 from 8 to 4
move 2 from 4 to 5
move 4 from 6 to 5
move 11 from 4 to 3
move 9 from 4 to 6
move 2 from 5 to 9
move 4 from 7 to 1
move 2 from 6 to 1
move 1 from 5 to 3
move 6 from 8 to 6
move 8 from 6 to 2
move 1 from 1 to 6
move 3 from 1 to 3
move 1 from 5 to 3
move 1 from 5 to 9
move 5 from 7 to 2
move 2 from 9 to 6
move 4 from 8 to 6
move 1 from 7 to 2
move 1 from 5 to 4
move 12 from 3 to 4
move 3 from 3 to 1
move 3 from 6 to 8
move 1 from 9 to 3
move 6 from 2 to 6
move 2 from 3 to 2
move 10 from 2 to 7
move 2 from 1 to 9
move 2 from 1 to 6
move 1 from 1 to 4
move 9 from 7 to 9
move 3 from 8 to 7
move 7 from 4 to 8
move 2 from 7 to 4
move 4 from 8 to 1
move 5 from 8 to 2
move 3 from 1 to 3
move 1 from 8 to 7
move 3 from 3 to 7
move 4 from 2 to 6
move 8 from 4 to 2
move 5 from 2 to 5
move 11 from 9 to 7
move 2 from 5 to 7
move 16 from 7 to 8
move 5 from 8 to 7
move 1 from 4 to 3
move 3 from 5 to 1
move 11 from 6 to 5
move 7 from 5 to 4
move 5 from 7 to 4
move 1 from 3 to 7
move 2 from 5 to 4
move 10 from 4 to 8
move 14 from 6 to 3
move 1 from 5 to 9
move 1 from 6 to 5
move 2 from 2 to 9
move 2 from 1 to 3
move 2 from 5 to 3
move 2 from 7 to 6
move 2 from 1 to 4
move 1 from 2 to 3
move 19 from 3 to 6
move 3 from 9 to 2
move 4 from 2 to 6
move 6 from 6 to 7
move 13 from 6 to 2
move 14 from 8 to 1
move 6 from 4 to 3
move 5 from 7 to 8
move 3 from 6 to 3
move 2 from 8 to 2
move 2 from 6 to 8
move 4 from 1 to 8
move 13 from 8 to 4
move 10 from 4 to 7
move 1 from 4 to 5
move 1 from 5 to 1
move 3 from 6 to 5
move 3 from 8 to 9
move 9 from 3 to 1
move 3 from 5 to 8
move 3 from 9 to 6
move 3 from 8 to 7
move 1 from 6 to 9
move 1 from 9 to 4
move 9 from 2 to 8
move 2 from 2 to 6
move 14 from 7 to 1
move 31 from 1 to 5
move 3 from 1 to 7
move 4 from 6 to 8
move 24 from 5 to 3
move 2 from 8 to 1
move 1 from 8 to 5
move 2 from 1 to 7
move 3 from 7 to 6
move 6 from 8 to 6
move 2 from 4 to 2
move 1 from 4 to 3
move 2 from 2 to 7
move 6 from 6 to 7
move 4 from 8 to 6
move 7 from 6 to 2
move 12 from 7 to 5
move 4 from 2 to 8
move 1 from 2 to 4
move 1 from 4 to 6
move 1 from 6 to 7
move 1 from 7 to 3
move 3 from 3 to 8
move 17 from 3 to 5
move 4 from 3 to 6
move 35 from 5 to 3
move 2 from 2 to 6
move 1 from 5 to 9
move 9 from 3 to 7
move 6 from 8 to 1
move 4 from 2 to 6
move 4 from 6 to 9
move 20 from 3 to 9
move 22 from 9 to 7
move 1 from 8 to 6
move 29 from 7 to 5
move 4 from 6 to 8
move 6 from 1 to 8
move 2 from 7 to 3
move 1 from 6 to 5
move 2 from 3 to 9
move 1 from 9 to 3
move 4 from 5 to 6
move 18 from 5 to 1
move 7 from 3 to 1
move 1 from 3 to 6
move 3 from 5 to 1
move 1 from 3 to 9
move 4 from 5 to 2
move 10 from 8 to 7
move 2 from 9 to 3
move 1 from 3 to 5
move 21 from 1 to 9
move 1 from 3 to 2
move 1 from 2 to 9
move 15 from 9 to 3
move 4 from 7 to 1
move 2 from 6 to 1
move 7 from 9 to 1
move 1 from 5 to 4
move 1 from 4 to 6
move 6 from 3 to 9
move 3 from 6 to 5
move 19 from 1 to 6
move 8 from 3 to 6
move 1 from 3 to 7
move 20 from 6 to 7
move 1 from 2 to 6
move 6 from 9 to 8
move 2 from 9 to 4
move 1 from 1 to 3
move 1 from 2 to 9
move 3 from 5 to 6
move 2 from 7 to 3
move 2 from 9 to 7
move 1 from 4 to 8
move 4 from 8 to 9
move 4 from 7 to 1
move 2 from 1 to 7
move 1 from 3 to 2
move 2 from 8 to 9
move 6 from 6 to 2
move 1 from 8 to 1
move 1 from 5 to 7
move 4 from 2 to 9
move 1 from 3 to 5
move 5 from 6 to 1
move 1 from 4 to 2
move 1 from 9 to 6
move 1 from 9 to 6
move 4 from 6 to 3
move 7 from 9 to 7
move 8 from 7 to 2
move 1 from 5 to 8
move 5 from 2 to 9
move 3 from 2 to 5
move 6 from 1 to 3
move 17 from 7 to 6
move 1 from 8 to 2
move 2 from 1 to 7
move 5 from 9 to 4
move 4 from 3 to 8
move 3 from 4 to 1
move 1 from 9 to 2
move 4 from 2 to 1
move 1 from 8 to 4
move 1 from 5 to 9
move 1 from 2 to 3
move 3 from 3 to 2
move 10 from 7 to 6
move 3 from 4 to 1
move 5 from 3 to 2
move 4 from 1 to 5
move 3 from 8 to 6
move 12 from 6 to 4
move 1 from 9 to 3
move 1 from 5 to 2
move 3 from 1 to 6
move 12 from 6 to 8
move 3 from 1 to 5
move 2 from 4 to 3
move 5 from 8 to 7
move 7 from 5 to 3
move 3 from 7 to 9
move 1 from 5 to 8
move 5 from 3 to 7
move 10 from 6 to 5
move 2 from 7 to 5
move 8 from 2 to 9
move 5 from 3 to 9
move 9 from 5 to 1
move 5 from 7 to 4
move 15 from 9 to 5
move 1 from 2 to 5
move 1 from 8 to 5
move 6 from 4 to 1
move 2 from 2 to 9
move 18 from 5 to 8
move 18 from 8 to 3
move 16 from 3 to 4
move 3 from 5 to 8
move 1 from 9 to 2
move 3 from 1 to 7
move 3 from 8 to 2
move 3 from 7 to 9
move 2 from 3 to 4
move 3 from 9 to 8
move 11 from 1 to 6
move 2 from 9 to 4
move 3 from 6 to 9
move 8 from 6 to 4
move 26 from 4 to 7
move 1 from 2 to 5
move 1 from 5 to 2
move 3 from 9 to 3
move 21 from 7 to 6
move 4 from 2 to 4
move 1 from 2 to 3
move 5 from 7 to 6
move 8 from 8 to 1
move 1 from 3 to 7
move 9 from 1 to 4
move 1 from 7 to 4
move 20 from 4 to 7
move 1 from 8 to 5
move 2 from 4 to 8
move 1 from 4 to 9
move 3 from 8 to 9
move 1 from 5 to 8
move 2 from 3 to 1
move 4 from 7 to 8
move 3 from 7 to 5
move 1 from 1 to 7
move 4 from 8 to 3
move 3 from 5 to 6
move 1 from 8 to 4
move 1 from 1 to 8
move 28 from 6 to 4
move 1 from 6 to 1
move 2 from 7 to 8
move 1 from 8 to 7
move 1 from 8 to 1
move 2 from 1 to 9
move 3 from 9 to 3
move 12 from 7 to 5
move 7 from 3 to 1
move 1 from 3 to 6
move 26 from 4 to 9
move 1 from 6 to 3
move 1 from 3 to 6
move 1 from 8 to 6
move 1 from 7 to 8
move 1 from 6 to 3
move 3 from 1 to 3
move 5 from 4 to 3
move 28 from 9 to 4
move 2 from 1 to 5
move 22 from 4 to 1
move 3 from 5 to 3
move 5 from 5 to 7
move 10 from 1 to 6
move 1 from 5 to 2
move 3 from 5 to 3
move 2 from 5 to 9
move 3 from 9 to 7
move 2 from 4 to 5
move 1 from 5 to 4
move 4 from 3 to 8
move 1 from 5 to 7
move 9 from 6 to 5
move 1 from 7 to 6
move 1 from 6 to 5
move 2 from 6 to 9
move 3 from 5 to 1
move 13 from 1 to 3
move 7 from 7 to 5
move 1 from 2 to 9
move 3 from 8 to 2
move 1 from 7 to 2
move 3 from 4 to 3
move 19 from 3 to 8
move 5 from 3 to 7
move 1 from 7 to 1
move 19 from 8 to 6
move 5 from 1 to 4
move 5 from 5 to 2
move 2 from 2 to 7
move 3 from 4 to 1
move 6 from 5 to 7
move 2 from 8 to 7
move 2 from 2 to 7
move 3 from 3 to 5
move 5 from 7 to 6
move 6 from 6 to 1
move 2 from 5 to 1
move 2 from 4 to 3
move 1 from 5 to 8";