use crate::utils::read_input_to_string;

type Action = (char, f32);

struct Boat {
    dir: f32,
    position: (f32, f32),
}

pub fn day12() {
    println!("DAY 12");
    part1();
}

fn part1() {
    let actions = read_actions();
    let mut boat = Boat { dir: 0., position: (0., 0.) };
    onward_and_forward(actions, &mut boat);
    println!("Final position has distance: {}",
        manhattan_distance(boat));
}

fn onward_and_forward(actions: Vec<Action>, boat: &mut Boat) {
    for (code, val) in actions {
        match code {
            'N' => boat.position.1 += val,
            'S' => boat.position.1 -= val,
            'E' => boat.position.0 += val,
            'W' => boat.position.0 -= val,
            'L' => boat.dir += val.to_radians(),
            'R' => boat.dir -= val.to_radians(),
            'F' => move_forward(boat, val),
            _ => panic!("Eoh no"),
        }
    }
}

fn move_forward(boat: &mut Boat, distance: f32) {
    boat.position.0 += distance * boat.dir.cos();
    boat.position.1 += distance * boat.dir.sin();
}

fn manhattan_distance(boat: Boat) -> f32 {
    boat.position.0.abs() + boat.position.1.abs()
}

fn read_actions() -> Vec<Action> {
    read_input_to_string(String::from("day12"))
        .lines()
        .map(|action| {
            (
                action.chars().next().unwrap(),
                action
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<f32>()
                    .unwrap(),
            )
        })
        .collect()
}
