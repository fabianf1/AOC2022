//
fn get_input(path: &str) -> Vec<((i32, i32), (i32, i32))>{
    // Have I mapped enough?
    std::fs::read_to_string(path)
    .unwrap() 
    .lines() 
    .map(|l| l.split_once(',').unwrap())
    .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
    .map(|((a,b),(c,d))| ((a.parse().unwrap(), b.parse().unwrap()),(c.parse().unwrap(), d.parse().unwrap())))
    .collect()
}
//
fn num_pair_contained(input: &Vec<((i32, i32), (i32, i32))>) -> usize{
    input.iter().filter(|((a,b),(c,d))| (c>=a && d<=b) || (a>=c && b<=d)).count()
}
// 
fn num_pair_overlap(input: &Vec<((i32, i32), (i32, i32))>) -> usize{
    input.iter().filter(|((a,b),(c,d))| (a>=c && a<=d) || (b>=c && b<=d) || (c>=a && c<=b) || (d>=a && d<=b)).count()
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");

    // Part 1
    assert!(num_pair_contained(&test) == 2,"Part 1 test failed");
    println!("Part 1: {:?}", num_pair_contained(&data));

    // Part 2
    assert!(num_pair_overlap(&test) == 4, "Part 2 test failed");
    println!("Part 2: {:?}", num_pair_overlap(&data));
}