use std::io::BufRead;

use anyhow::Ok;

use crate::read_input;

pub struct Report {
    data: Vec<Vec<i32>>,
}

impl Report {
    pub fn new_from_data() -> anyhow::Result<Report> {
        let reader = read_input("2024/2.txt")?;

        let mut data = vec![];
        for line in reader.lines() {
            let s = line?;
            let r = s
                .split(' ')
                .filter_map(|v| {
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.parse::<i32>().unwrap_or(0))
                    }
                })
                .collect::<Vec<i32>>();

            data.push(r);
        }

        Ok(Report { data })
    }

    fn is_safe(reports: &[i32]) -> bool {
        let mut level = None;

        for w in reports.windows(2) {
            let delta = w[1] - w[0];
            if delta == 0 || delta.abs() > 3 {
                return false;
            }

            let current_level = if delta > 0 { Some(true) } else { Some(false) };
            if level.is_none() {
                level = current_level;
            };

            if level != current_level {
                return false;
            }
        }

        true
    }

    pub fn sum_of_safe_reports(&self) -> i32 {
        self.data.iter().filter(|d| Self::is_safe(d)).count() as i32
    }

    pub fn sum_of_dampened_reports(&self) -> i32 {
        let mut total = 0;
        self.data.iter().for_each(|v| {
            if Self::is_safe(v) {
                total += 1;
                return;
            }

            for (k, _n) in v.iter().enumerate() {
                let mut p = v.clone();
                p.remove(k);

                if Self::is_safe(&p) {
                    total += 1;
                    return;
                };
            }
        });

        total
    }
}

fn part_one() -> anyhow::Result<i32> {
    let report = Report::new_from_data()?;

    Ok(report.sum_of_safe_reports())
}

fn part_two() -> anyhow::Result<i32> {
    let report = Report::new_from_data()?;

    Ok(report.sum_of_dampened_reports())
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_2_1() {
        let ans = part_one().unwrap();
        assert_eq!(663, ans);
    }

    #[test]
    fn test_2_2() {
        let ans = part_two().unwrap();
        assert_eq!(692, ans);
    }
}
