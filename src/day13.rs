use crate::utils::read_input_to_string;

pub fn day13() {
    println!("DAY 13");
    part1();
}

fn part1() {
    let (arrival_time, bus_ids) = read_input();
    let bus_ids: Vec<i32> = bus_ids
        .iter()
        .filter_map(|s| match s.as_str() {
            "x" => None,
            _ => Some(s.parse::<i32>().unwrap()),
        }).collect();

    let (closest_bus, waiting_time) = bus_ids
        .iter()
        .map(|interval| (interval, (interval - (arrival_time % interval)) % interval))
        .min_by_key(|(_, wait)| *wait)
        .unwrap();

    println!("Answer is: {}", closest_bus * waiting_time);
}

// fn part2() {
//     let (_, bus_ids) = read_input();
//     let bus_ids: Vec<(usize, i32)> = bus_ids.iter()
//         .enumerate()
//         .filter_map(|(i, s)| match s.as_str() {
//             "x" => None,
//             _ => Some((i, s.parse::<i32>().unwrap())),
//         }).collect();
// }

fn read_input() -> (i32, Vec<String>) {
    let input = read_input_to_string(String::from("day13"));
    let mut lines = input.lines();
    let arrival_time = lines.next().unwrap().parse::<i32>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (arrival_time, bus_ids)
}
