fn main() {
    //let input = include_str!("../input.example.txt");
    let input = include_str!("../input.txt");

    let sum = input
        .lines()
        .map(|l| (l.chars().find(|&x| x.is_digit(10)).unwrap(), l.chars().rfind(|&x| x.is_digit(10)).unwrap()))
        .map(|t| format!("{}{}", t.0, t.1).parse::<u16>().unwrap())
        .sum::<u16>();

    print!("{sum}");

}
