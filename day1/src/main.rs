use std::fs;

fn thicc_elf_ometer(input: &str) -> Vec<i32> {
    let data: Vec<&str> = input.split_terminator("\n\n").collect();
    let data: Vec<Vec<i32>> = data
        .iter()
        .map(|f| f.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    return data
        .into_iter()
        .map(|x| x.into_iter().reduce(|accum, value| accum + value).unwrap())
        .collect();
}

fn part1(data: &Vec<i32>) -> String {
    data.iter().max().unwrap().to_string()
}

fn part2(data: &Vec<i32>) -> String {
    let mut sorted_data = data.clone();
    sorted_data.sort();
    sorted_data.iter().rev().take(3).sum::<i32>().to_string()
}

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();
    let data = thicc_elf_ometer(&data);

    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::thicc_elf_ometer;
    const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_part1() {
        let expected = "24000".to_string();

        assert_eq!(part1(&thicc_elf_ometer(EXAMPLE)), expected);
    }

    #[test]
    fn test_part2() {
        let expected = "45000".to_string();

        assert_eq!(part2(&thicc_elf_ometer(EXAMPLE)), expected);
    }
}
