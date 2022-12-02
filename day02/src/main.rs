fn get_input(path: &str) -> Vec<[char; 2]>{
    std::fs::read_to_string(path)
    .unwrap() // Give me the result of the computation, and if there was an error, panic and stop the program.
    .lines() // An iterator over the lines of a string, as string slices.
    .map(|l| l.split_whitespace().map(|c:&str| c.parse::<char>().unwrap()).collect::<Vec<char>>().try_into().unwrap()) // Trying to put things into an array is not fun
    .collect() // Collect the results
}
//
fn calc_score(input: &Vec<[char; 2]>) -> i32{
    input.iter().map(|l| match l{
        ['A','X'] => 3 + 1,
        ['A','Y'] => 6 + 2,
        ['A','Z'] => 0 + 3,
        ['B','X'] => 0 + 1,
        ['B','Y'] => 3 + 2,
        ['B','Z'] => 6 + 3,
        ['C','X'] => 6 + 1,
        ['C','Y'] => 0 + 2,
        ['C','Z'] => 3 + 3,
        _ => panic!(),
    }).sum()
}
//
fn calc_score2(input: &Vec<[char; 2]>) -> i32{
    input.iter().map(|l| match l{
        ['A','X'] => 3 + 0,
        ['A','Y'] => 1 + 3,
        ['A','Z'] => 2 + 6,
        ['B','X'] => 1 + 0,
        ['B','Y'] => 2 + 3,
        ['B','Z'] => 3 + 6,
        ['C','X'] => 2 + 0,
        ['C','Y'] => 3 + 3,
        ['C','Z'] => 1 + 6,
        _ => panic!(),
    }).sum()
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");
    
    // Part 1
    assert!(calc_score(&test) == 15,"Part 1 test failed");
    println!("Part 1: {:?}", calc_score(&data));

    // Part 2
    assert!(calc_score2(&test) == 12, "Part 2 test failed");
    println!("Part 2: {:?}", calc_score2(&data));
}
