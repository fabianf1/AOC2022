//
fn get_input(path: &str) -> Vec<Vec<char>>{
    std::fs::read_to_string(path)
    .unwrap() 
    .lines() 
    .map(|l| l.chars().collect())
    .collect()
}
//
fn calc_prio_sum(input: &Vec<Vec<char>>) -> i32{
    let mut sum: i32 = 0;
    for sack in input{
        let mid = sack.len()/2;
        for i in 0..mid{
            if sack.iter().skip(mid).filter(|&n| *n == sack[i]).count() > 0 {
                sum += if sack[i] as i32 > 96 { sack[i] as i32 - 96 } else { sack[i] as i32 - 38 };
                break
            }
        }
    }
    return sum
}
fn calc_badge_sum(input: &Vec<Vec<char>>) -> i32{
    let mut sum: i32 = 0;
    for i in (0..input.len()).step_by(3){
        for j in 0..input[i].len(){
            if input[i+1].iter().filter(|&n| *n == input[i][j]).count() > 0 && input[i+2].iter().filter(|&n| *n == input[i][j]).count() > 0{
                sum += if input[i][j] as i32 > 96 { input[i][j] as i32 - 96 } else { input[i][j] as i32 - 38 };
                break
            }
        }
    }
    return sum
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");

    // Part 1
    assert!(calc_prio_sum(&test) == 157,"Part 1 test failed");
    println!("Part 1: {:?}", calc_prio_sum(&data));

    // Part 2
    assert!(calc_badge_sum(&test) == 70, "Part 2 test failed");
    println!("Part 2: {:?}", calc_badge_sum(&data));
}