use std::io::Read;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read_input() -> Vec<i32> {
    let mut file = std::fs::File::open("inputs/day1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Vec<i32> = Vec::new();
    for s in contents.lines() {
        v.push(s.parse::<i32>().unwrap());
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two() {
        let input = read_input();
        let (x, y) = find_sum_to(input, 2020);
        println!("The answer is: {}", x * y);
    }

    #[test]
    fn test_three() {
        let input = read_input();
        let (x, y, z) = find_three_sum_to(input, 2020);
        println!("The answer is: {}", x * y * z);
    }
}
