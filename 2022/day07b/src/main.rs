use std::collections::BTreeMap;

struct File {
    size: u32
}

fn main() {

    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let dirs_tree = build_tree(input);
    let sizes_tree = calculate_size_of_children(dirs_tree);

    const FILE_SYSTEM_SIZE:u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;
    let root_size = sizes_tree.values().next().unwrap().clone();
    let minimum_needed_space_to_delete = UPDATE_SIZE - (FILE_SYSTEM_SIZE - root_size);
    
    let ans = sizes_tree
        .values()
        .filter(|&size| *size >= minimum_needed_space_to_delete)
        .min()
        .unwrap();

    println!("{ans}")
}

fn build_tree(terminal_output: &str) -> BTreeMap<String, Vec<File>> {

    let mut dirs: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut file_path: Vec<&str> = Vec::new();

    for cmd_and_cmd_output in terminal_output.split("$").skip(1) {
        let mut cmd_and_cmd_output_by_line: Vec<&str> = cmd_and_cmd_output.lines().collect();
        let cmd = cmd_and_cmd_output_by_line.remove(0).trim();

        if cmd == "ls" {

            // If a dir doesn't already exist, prep it for future adding of files
            dirs
                .entry(get_file_path_string(&file_path))
                .or_insert(Vec::new());

            let ls_output_lines = cmd_and_cmd_output_by_line;
            for ls_output_line in ls_output_lines {
                let (dir_identifier_or_size, _) = ls_output_line.split_once(" ").unwrap();

                // Don't do anything for dirs
                if dir_identifier_or_size == "dir" { }
                // Add files to the dir
                else {
                    let file = File{
                        size: dir_identifier_or_size.parse::<u32>().unwrap()
                    };

                    dirs
                        .entry(get_file_path_string(&file_path))
                        .and_modify(|folder| folder.push(file));
                }
            }
        }
        else {
            let (_, dir_name) = cmd.split_once(" ").unwrap();

            if dir_name == "/" {
                // root
                file_path.clear();
                file_path.push("root")
            }
            else if dir_name == ".." {
                // move up
                file_path.pop();
            }
            else {
                // move down
                file_path.push(dir_name);
            }
        }
    }

    return dirs;
}

fn calculate_size_of_children(dirs_tree: BTreeMap<String, Vec<File>>) -> BTreeMap<String, u32>{
    let mut sizes_tree: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in dirs_tree.iter() {
        let size_of_files_in_dir = files.iter().map(|f| f.size).sum::<u32>();
        let dirs_of_path = path.split("/").collect::<Vec<&str>>();
        for path_part_index in 0..dirs_of_path.len() {
            let path_part = get_file_path_string(&dirs_of_path[..=path_part_index].to_vec());
            sizes_tree
                .entry(path_part)
                .and_modify(|v| *v += size_of_files_in_dir)
                .or_insert(size_of_files_in_dir);
        }
    }

    return sizes_tree;
}

fn get_file_path_string(file_path: &Vec<&str>) -> String {
    let mut file_path_string = String::new();
    for (i,part) in file_path.clone().iter().enumerate() {
        file_path_string += part;
        if i < file_path.len()-1 {
            file_path_string += "/";
        }
    }
    return file_path_string;
}