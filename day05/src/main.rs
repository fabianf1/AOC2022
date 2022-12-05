//
fn get_input(path: &str) -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
    // Part 1
    // Could also just have used an iterator that steps by 4...
    let part1: Vec<Vec<char>> = std::fs::read_to_string(path)
        .unwrap()
        .split("\r\n\r\n")
        .next()
        .unwrap()
        .lines()
        .filter(|l| l.chars().find(|c| *c == '[') != None) // Remove the last line
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c[1])
                .collect()
        })
        .rev() // Makes it much easier later!
        .collect();
    // Part 2
    let part2: Vec<[usize; 3]> = std::fs::read_to_string(path)
        .unwrap()
        .split("\r\n\r\n")
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter(|s| !s.parse::<i32>().is_err()) // Remove the move, from and to
                .map(|l| l.parse().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect();
    //
    return (part1, part2);
}
// Instead of d1, d2. Should have a vector of top-positions
fn find_top_after_move((mut crs, ops): (Vec<Vec<char>>, Vec<[usize; 3]>), version: i32) -> String {
    // Move crates
    for op in ops {
        let fr = op[1] - 1;
        let to = op[2] - 1;
        // Find stack depths
        let mut d1 = find_depth(&crs, fr) - if version == 9001 { op[0] } else { 0 };
        let mut d2 = find_depth(&crs, to);
        // Execute move
        for _i in 0..op[0] {
            while crs.len() <= d2 {
                crs.push(vec![' '; crs[0].len()]);
            }
            // Operation
            d1 -= if version == 9000 { 1 } else { 0 };
            crs[d2][to] = crs[d1][fr];
            crs[d1][fr] = ' ';
            d1 += if version == 9001 { 1 } else { 0 };
            d2 += 1;
        }
    }
    // Get top positions
    let mut output = String::new();
    for i in 0..crs[0].len() {
        let d = find_depth(&crs, i) - 1;
        output.push(crs[d][i]);
    }
    //
    return output;
}
// Finds first empty position!
fn find_depth(crs: &Vec<Vec<char>>, pos: usize) -> usize {
    for i in 0..crs.len() {
        if crs[i][pos] == ' ' {
            return i;
        }
    }
    return crs.len();
}
//
fn main() {
    // Part 1
    assert!(
        find_top_after_move(get_input("Data/Test.txt"), 9000) == "CMZ",
        "Part 1 test failed"
    );
    println!(
        "Part 1: {:?}",
        find_top_after_move(get_input("Data/Data.txt"), 9000)
    );

    // Part 2
    assert!(
        find_top_after_move(get_input("Data/Test.txt"), 9001) == "MCD",
        "Part 2 test failed"
    );
    println!(
        "Part 2: {:?}",
        find_top_after_move(get_input("Data/Data.txt"), 9001)
    );
}
