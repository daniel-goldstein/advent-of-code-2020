use std::io::Read;

pub fn read_input_to_string(input_name: String) -> String {
    let mut file = std::fs::File::open(format!("./inputs/{}.txt", input_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
