struct Solution {}

const REGULAR_YEAR_DAYS_IN_MONTHS: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug, Clone)]
struct Date {
    year: u16,
    month: u16,
    day: u16,
}

fn is_leap(year: u16) -> bool {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

fn parse_date(date: String) -> Date {
    let split: Vec<&str> = date.split('-').collect();

    return Date {
        year: split[0].parse().unwrap(),
        month: split[1].parse().unwrap(),
        day: split[2].parse().unwrap(),
    };
}

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let parsed = parse_date(date);

        let mut days_if_regular: u16 = REGULAR_YEAR_DAYS_IN_MONTHS
            [0usize..parsed.month as usize - 1]
            .iter()
            .map(|&x| x as u16)
            .sum();

        if days_if_regular > 58 && is_leap(parsed.year) {
            days_if_regular += 1
        }

        return (days_if_regular + parsed.day) as i32;
    }
}

fn main() {
    let date: String = "2019-01-09".to_string();
    assert!(Solution::day_of_year(date) == 9);

    let date: String = "2019-02-10".to_string();
    assert!(Solution::day_of_year(date) == 41);

    let date: String = "2004-03-01".to_string();
    assert!(Solution::day_of_year(date) == 61);

    println!("passed");
}
