use crate::utils::read_input_to_string;

pub fn day11() {
    let seat_chart = read_input_to_string(String::from("day11"))
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // println!(
    //     "Final occupied seats: {}",
    //     seats_filled(simulate(seat_chart.clone()))
    // );
    println!(
        "Final occupied seats with sight: {}",
        seats_filled(simulate(seat_chart.clone()))
    );
}

fn simulate(seat_chart: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let next = step_simulation(&seat_chart);
    if next == seat_chart {
        return next;
    }

    simulate(next)
}

fn step_simulation(seat_chart: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_chart = vec![vec!['.'; seat_chart[0].len()]; seat_chart.len()];
    for i in 0..seat_chart.len() {
        for j in 0..seat_chart[0].len() {
            // let neighbors = neighboring_seats(seat_chart, i, j);
            let neighbors = neighboring_seats_with_sight(seat_chart, i, j);
            let num_occupied_neighbors = neighbors.iter().filter(|&&s| s == '#').count();
            if seat_chart[i][j] == 'L' && num_occupied_neighbors == 0 {
                new_chart[i][j] = '#';
            } else if seat_chart[i][j] == '#' && num_occupied_neighbors >= 5 {
                new_chart[i][j] = 'L';
            } else {
                new_chart[i][j] = seat_chart[i][j];
            }
        }
    }

    new_chart
}

fn neighboring_seats(seat_chart: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let mut neighbors = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            let row_offset = (row as i32) + i;
            let col_offset = (col as i32) + j;
            if !(i == 0 && j == 0) && in_bounds(seat_chart, row_offset, col_offset) {
                neighbors.push(seat_chart[row_offset as usize][col_offset as usize]);
            }
        }
    }

    neighbors
}

fn neighboring_seats_with_sight(seat_chart: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<char> {
    let mut neighbors = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            if !(i == 0 && j == 0) {
                let row_offset = (row as i32) + i;
                let col_offset = (col as i32) + j;
                if let Some(seat) = first_seat_in_direction(seat_chart, row_offset, col_offset, i, j) {
                    neighbors.push(seat);
                }
            }
        }
    }

    neighbors
}

fn first_seat_in_direction(
    seat_chart: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    i: i32,
    j: i32,
) -> Option<char> {
    if !in_bounds(seat_chart, row, col) {
        return None
    }

    let seat = seat_chart[row as usize][col as usize];
    if seat == 'L' || seat == '#' {
        return Some(seat)
    }

    first_seat_in_direction(seat_chart, row + i, col + j, i, j)
}

fn in_bounds(seat_chart: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    0 <= row && row < (seat_chart.len() as i32) && 0 <= col && col < (seat_chart[0].len() as i32)
}

fn seats_filled(seat_chart: Vec<Vec<char>>) -> usize {
    seat_chart
        .iter()
        .flat_map(|row| row.iter())
        .filter(|seat| **seat == '#')
        .count()
}
