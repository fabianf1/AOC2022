// Output: Vector of 32 bit integers
// Arrays do have a fixed size in Rust!
fn get_input(path: &str) -> Vec<i32>{
    let mut data:Vec<i32> = Vec::new(); // Will be modified therefore "mut"
    for line in std::fs::read_to_string(path).unwrap().lines(){
        if line.chars().count() > 0 {
            data.push(line.parse::<i32>().unwrap());
        }
        else{
            data.push(-1);
        }
        //println!("{:?}", line.chars().count());
    }
    data.push(-1); // Helpful later
    
    return data;

    // Errors because of the empty line
    /*std::fs::read_to_string("Data/Test.txt")
    .unwrap() // Give me the result of the computation, and if there was an error, panic and stop the program.”
    .lines() // An iterator over the lines of a string, as string slices.
    .map(|l|l.parse::<i32>().unwrap() )
    .collect() // Collect the result of the iterator*/

    // Return can be done by using "return val;"
    // Or if it is the last statement in a function just do "val"
}

fn get_max_cal(data: Vec<i32>) -> i32{
    let mut max_cal = 0;
    let mut elf_cal = 0;
    for cal in data{
        if cal != -1{
            elf_cal += cal;
        }
        else{
            if elf_cal > max_cal{
                max_cal = elf_cal
            }
            elf_cal = 0
        }
    }
    return max_cal
}

fn get_top3(data: Vec<i32>) -> i32{
    let mut top3: [i32; 3] = [0; 3]; 
    let mut elf_cal = 0;
    for cal in data{
        if cal != -1{
            elf_cal += cal;
        }
        else{
            if elf_cal > top3[2] {
                if elf_cal > top3[1] {
                    top3[2] = top3[1];
                    if elf_cal > top3[0] {
                        top3[1] = top3[0];
                        top3[0] = elf_cal;
                    }
                    else{
                        top3[1] = elf_cal;
                    }
                }
                else{
                    top3[2] = elf_cal;
                }
            }
            elf_cal = 0;
        }
    }
    //
    let mut output:i32 = 0;
    for cal in top3{
        output += cal;
    }

    return output
}

fn main() {
    // Need way to not move data
    // Part 1
    println!("Test answer 1: {:?}", get_max_cal(get_input("Data/Test.txt")));
    println!("Part 1: {:?}", get_max_cal(get_input("Data/Data.txt")));

    // Part 2
    println!("Test answer 2: {:?}", get_top3(get_input("Data/Test.txt"))); 
    println!("Part 2: {:?}", get_top3(get_input("Data/Data.txt"))); 
}