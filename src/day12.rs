use crate::utils::read_input_to_string;

type Action = (char, f32);

pub fn day12() {
    println!("DAY 12");
    let actions = read_actions();
    part1(&actions);
    part2(&actions);
}

struct Boat {
    dir: f32,
    position: (f32, f32),
}

impl Boat {
    fn new() -> Self {
        Boat { dir: 0., position: (0., 0.) }
    }

    fn onward_and_forward(&mut self, actions: &[Action]) {
        for (code, val) in actions {
            match code {
                'N' => self.position.1 += val,
                'S' => self.position.1 -= val,
                'E' => self.position.0 += val,
                'W' => self.position.0 -= val,
                'L' => self.dir += val.to_radians(),
                'R' => self.dir -= val.to_radians(),
                'F' => self.move_forward(*val),
                _ => panic!("Eoh no"),
            }
        }
    }

    fn move_forward(&mut self, distance: f32) {
        self.position.0 += distance * self.dir.cos();
        self.position.1 += distance * self.dir.sin();
    }
}

fn part1(actions: &[Action]) {
    let mut boat = Boat::new();
    boat.onward_and_forward(actions);
    println!("Final position has distance: {}",
        manhattan_distance(boat.position));
}

struct Boat2 {
    waypoint: (f32, f32),
    position: (f32, f32),
}

impl Boat2 {
    fn new() -> Self {
        Boat2 { waypoint: (10., 1.), position: (0., 0.) }
    }

    fn onward_and_forward(&mut self, actions: &[Action]) {
        for (code, val) in actions {
            match code {
                'N' => self.waypoint.1 += val,
                'S' => self.waypoint.1 -= val,
                'E' => self.waypoint.0 += val,
                'W' => self.waypoint.0 -= val,
                'L' => self.rotate_waypoint(val.to_radians()),
                'R' => self.rotate_waypoint(-val.to_radians()),
                'F' => self.move_forward(*val),
                _ => panic!("whoops"),
            }
        }
    }

    fn rotate_waypoint(&mut self, radians: f32) {
        let (x, y) = self.waypoint;

        let x_new = radians.cos() * x - radians.sin() * y;
        let y_new = radians.sin() * x + radians.cos() * y;

        self.waypoint = (x_new, y_new);
    }

    fn move_forward(&mut self, times: f32) {
        self.position.0 += times * self.waypoint.0;
        self.position.1 += times * self.waypoint.1;
    }
}

fn part2(actions: &[Action]) {
    let mut boat = Boat2::new();
    boat.onward_and_forward(actions);
    println!("The actual distance is: {}",
        manhattan_distance(boat.position));
}

fn manhattan_distance(position: (f32, f32)) -> f32 {
    position.0.abs() + position.1.abs()
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
