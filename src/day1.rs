use crate::utils::read_input_to_string;

#[allow(dead_code)]
fn main() {
    let input = read_input();
    let (x, y) = find_sum_to(input, 2020);
    println!("The answer is: {}", x * y);

    let input = read_input();
    let (x, y, z) = find_three_sum_to(input, 2020);
    println!("The answer is: {}", x * y * z);
}

fn find_sum_to(nums: Vec<i32>, sum: i32) -> (i32, i32) {
    for &i in nums.iter() {
        for &j in nums.iter() {
            if i + j == sum {
                return (i, j)
            }
        }
    }
    panic!("Couldn't find desired numbers");
}

fn find_three_sum_to(nums: Vec<i32>, sum: i32) -> (i32, i32, i32) {
    for &i in nums.iter() {
        for &j in nums.iter() {
            for &k in nums.iter() {
                if i + j + k == sum {
                    return (i, j, k)
                }
            }
        }
    }
    panic!("Couldn't find desired numbers");
}

fn read_input() -> Vec<i32> {
    let contents = read_input_to_string(String::from("day1"));
    let mut v: Vec<i32> = Vec::new();
    for s in contents.lines() {
        v.push(s.parse::<i32>().unwrap());
    }
    v
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    fn test() {
        main();
    }
}
