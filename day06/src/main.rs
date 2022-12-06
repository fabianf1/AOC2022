use std::collections::HashSet;
//
fn get_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
//
fn find_start(input: &String, num: usize) -> usize {
    for i in (num - 1)..input.len() {
        if input.chars().skip(i - (num - 1)).take(num).collect::<HashSet<char>>().len() == num {
            return i + 1;
        }
    }
    return 0;
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");

    // Part 1
    assert!(find_start(&test, 4) == 7, "Part 1 test failed");
    println!("Part 1: {:?}", find_start(&data, 4));

    // Part 2
    assert!(find_start(&test, 14) == 19, "Part 2 test failed");
    println!("Part 2: {:?}", find_start(&data, 14));
}
