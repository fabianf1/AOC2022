fn get_input(path: &str) -> Vec<Vec<u32>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.into()).map(|n: u32| n - 48).collect())
        .collect()
}
//
fn num_visible(input: &Vec<Vec<u32>>) -> usize{
    let mut num = (input.len()-1) * 4; // Edges
    for i in 1..(input.len()-1){
        for j in 1..(input[0].len()-1){
            if  input[i].iter().take(j).filter(|&n| *n >= input[i][j]).count() ==0 || // Left
                input[i].iter().skip(j+1).filter(|&n| *n >= input[i][j]).count() ==0 || // Right
                input.iter().take(i).map(|r| r[j]).filter(|&n| n >= input[i][j]).count() ==0||  // Top
                input.iter().skip(i+1).map(|r| r[j]).filter(|&n| n >= input[i][j]).count() == 0{ // Bottom
                num += 1;
            }
        }
    }
    return num
}
//
fn best_score(input: &Vec<Vec<u32>>) -> usize{
    let mut score: usize = 0;
    for i in 1..(input.len()-1){
        for j in 1..(input[0].len()-1){
            let new_score = (input[i].iter().take(j).rev().position(|&n| n >= input[i][j]).unwrap_or(j-1) + 1) * 
                                (input[i].iter().skip(j+1).position(|&n| n >= input[i][j]).unwrap_or(input[0].len()-(j+1)-1) + 1) * 
                                (input.iter().take(i).rev().map(|r| r[j]).position(|n| n >= input[i][j]).unwrap_or(i-1) + 1) * 
                                (input.iter().skip(i+1).map(|r| r[j]).position(|n| n >= input[i][j]).unwrap_or(input.len()-(i+1)-1) + 1);
            if new_score > score{
                score = new_score;
            }
        }
    }
    return score
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");

    // Part 1
    assert!(num_visible(&test) == 21, "Part 1 test failed");
    println!("Part 1: {:?}", num_visible(&data));

    // Part 2
    assert!(best_score(&test) == 8, "Part 2 test failed");
    println!("Part 2: {:?}", best_score(&data));
}
