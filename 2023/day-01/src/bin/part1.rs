fn main() {
    let input = include_str!("./input1.txt");
    
    if let Ok(sum) = part1(input) {
        println!("Sum: {}", sum);
    } else {
        eprintln!("Error calculating sum");
    }
}

fn part1(input: &str) -> Result<i32, String> {
    let modified_vec: Vec<String> = input
        .lines()
        .filter_map(|s| extract_numericals(s).and_then(combine_first_last))
        .collect();

    sum(&modified_vec)
}

fn extract_numericals(input: &str) -> Option<String> {
    let result: String = input.chars().filter(|c| c.is_numeric()).collect();
    Some(result)
}

fn combine_first_last(input: String) -> Option<String> {
    let length = input.len();
    if length >= 1 {
        Some(format!("{}{}", &input[0..1], &input[length - 1..]))
    } else {
        None
    }
}

fn sum(input: &[String]) -> Result<i32, String> {
    let sum: Result<i32, _> = input
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .try_fold(0_i32, |acc, value| {
            acc.checked_add(value).ok_or_else(|| "Sum overflowed i32".to_string())
        });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_file_input() {
        let input = include_str!("./input1.txt");
        let result = part1(input);
        assert_eq!(result, Ok(54708));
    }
}
