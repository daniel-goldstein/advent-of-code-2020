use crate::utils::read_input_to_string;

type Terrain = Vec<Vec<Square>>;

pub enum Square {
    Open,
    Tree,
}

pub fn day3() {
    println!("DAY 3");

    let terrain = read_input();
    println!(
        "With 3 to the right and 1 down, you hit {} trees.",
        how_many_trees_hit(&terrain, 3, 1)
    );

    let total = how_many_trees_hit(&terrain, 1, 1)
        * how_many_trees_hit(&terrain, 3, 1)
        * how_many_trees_hit(&terrain, 5, 1)
        * how_many_trees_hit(&terrain, 7, 1)
        * how_many_trees_hit(&terrain, 1, 2);
    println!(
        "The product of all products hit across all slopes is {}.",
        total
    );
}

fn how_many_trees_hit(terrain: &Terrain, right_offset: usize, down_offset: usize) -> u32 {
    terrain.iter()
        .step_by(down_offset)
        .enumerate()
        .fold(0, |total, (j, row)| {
            match row[(right_offset * j) % row.len()] {
                Square::Open => total,
                Square::Tree => total + 1,
            }
        })
}

fn read_input() -> Terrain {
    read_input_to_string(String::from("day3"))
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '#' => Square::Tree,
                    '.' => Square::Open,
                    _ => panic!("Malformed terrain"),
                })
                .collect::<Vec<Square>>()
        })
        .collect::<Terrain>()
}
