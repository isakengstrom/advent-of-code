
fn main() {

    //let mut input = include_str!("../input.example.txt").to_string();
    let mut input = include_str!("../input.txt").to_string();

    // input may have a disgusting bom char in the beginning. If so, remove it  
    if input.starts_with('\u{feff}') {
        input.remove(0);
    }
    
    let elf_pairs: Vec<((u32, u32), (u32, u32))> = input
        .lines()
        .map(|pair|
            pair
                .split(',')
                .map(|section| section
                    .split('-')
                    .map(|section_limit| section_limit.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
                )
                .map(|section_limits| (section_limits[0], section_limits[1]))
                .collect::<Vec<(u32, u32)>>()
        )
        .map(|pair_sections| (pair_sections[0], pair_sections[1]))
        .collect();
   
    let mut count = 0;
    for pair in elf_pairs {
        let elf_0 = pair.0;
        let elf_1 = pair.1;
        
        let elf_0_sections = elf_0.0..elf_0.1+1;
        
        if elf_0_sections.contains(&elf_1.0) && elf_0_sections.contains(&elf_1.1) {
            count += 1;
            continue; 
        }
        
        let elf_1_sections = elf_1.0..elf_1.1+1;
        
        if elf_1_sections.contains(&elf_0.0) && elf_1_sections.contains(&elf_0.1) {
            count += 1;
        }
    }
    
    println!("{}", count)

}
