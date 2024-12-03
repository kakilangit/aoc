use std::{collections::HashMap, io::BufRead};

use anyhow::Ok;

use crate::read_input;

struct Location {
    left: Vec<i32>,
    right: Vec<i32>,
    right_counter: HashMap<i32, i32>,
}

impl Location {
    fn new_from_data() -> anyhow::Result<Self> {
        let reader = read_input("2024/1.txt")?;

        let mut left = vec![];
        let mut right = vec![];
        let mut right_counter = HashMap::new();
        for line in reader.lines() {
            let s = line?;
            let split = s.split(' ').collect::<Vec<_>>();
            let l = split.first().unwrap_or(&"").parse::<i32>()?;
            let r = split.last().unwrap_or(&"").parse::<i32>()?;

            if let Some(v) = right_counter.get_mut(&r) {
                *v += 1;
            } else {
                right_counter.insert(r, 1);
            }

            left.push(l);
            right.push(r);
        }

        left.sort();
        right.sort();

        Ok(Self {
            left,
            right,
            right_counter,
        })
    }

    fn sum_of_difference(&self) -> anyhow::Result<i32> {
        Ok(self
            .left
            .iter()
            .enumerate()
            .map(|(k, v)| (v - self.right.get(k).unwrap()).abs())
            .sum::<i32>())
    }

    fn sum_of_similarities(&self) -> anyhow::Result<i32> {
        Ok(self
            .left
            .iter()
            .map(|v| (v * self.right_counter.get(v).unwrap_or(&0)).abs())
            .sum::<i32>())
    }
}

fn part_one() -> anyhow::Result<i32> {
    let location = Location::new_from_data()?;

    location.sum_of_difference()
}

fn part_two() -> anyhow::Result<i32> {
    let location = Location::new_from_data()?;

    location.sum_of_similarities()
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let ans = part_one().unwrap();
        assert_eq!(2367773, ans)
    }

    #[test]
    fn test_part_two() {
        let ans = part_two().unwrap();
        assert_eq!(21271939, ans)
    }
}
