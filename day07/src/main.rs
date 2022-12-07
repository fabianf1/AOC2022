struct Folder {
    folders: Vec<Folder>,
    file_size: usize,
    size: usize,
}
impl Folder {
    pub fn new() -> Folder {
        Folder {
            folders: vec![],
            file_size: 0,
            size: 0,
        }
    }
    //
    fn pop_size(&mut self) -> usize {
        self.size = self.file_size;
        self.size += self.folders.iter_mut().map(|f| f.pop_size()).sum::<usize>();
        self.size
    }
    //
    fn size_max(&self, max_size: usize) -> usize {
        let children = self
            .folders
            .iter()
            .fold(0, |acc, f| acc + f.size_max(max_size));
        if self.size > max_size {
            children
        } else {
            self.size + children
        }
    }
    //
    fn at_least(&self, val: usize) -> usize {
        self.folders.iter().fold(self.size, |acc, f| {
            let child = f.at_least(val);
            if child >= val && child < acc {
                child
            } else {
                acc
            }
        })
    }
}
//
fn get_input(path: &str) -> Folder {
    let mut root = Folder::new();
    let data = std::fs::read_to_string(path).unwrap();

    proc_input(&mut root, &mut data.lines());
    root.pop_size();

    return root;
}
//
fn proc_input(folder: &mut Folder, lines: &mut std::str::Lines<'_>) {
    loop {
        let line = lines.next();
        if line == None {
            break;
        }
        //
        let cmd: Vec<&str> = line.unwrap().split_whitespace().collect();
        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                if cmd[2] == ".." {
                    return;
                } else if cmd[2] != "/" {
                    let mut new_folder = Folder::new();
                    proc_input(&mut new_folder, lines);
                    folder.folders.push(new_folder);
                }
            }
        } else if cmd[0] != "dir" {
            folder.file_size += cmd[0].parse::<usize>().unwrap();
        }
    }
}
//
fn get_min_del(folder: &Folder) -> usize {
    let req = 30_000_000 - (70_000_000 - folder.size);
    folder.at_least(req)
}
//
fn main() {
    // Load data
    let test = get_input("Data/Test.txt");
    let data = get_input("Data/Data.txt");

    // Part 1
    assert!(test.size_max(100000) == 95437, "Part 1 test failed");
    println!("Part 1: {:?}", data.size_max(100000));

    // Part 2
    assert!(get_min_del(&test) == 24933642, "Part 2 test failed");
    println!("Part 2: {:?}", get_min_del(&data));
}
