use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

use crate::read_input;

#[derive(Debug)]
pub struct ElfPrinter {
    rules: HashMap<i32, Vec<i32>>,
    updates: Vec<Vec<i32>>,
}

impl ElfPrinter {
    pub fn new_from_data() -> anyhow::Result<Self> {
        let reader = read_input("2024/5.txt")?;

        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut updates: Vec<Vec<i32>> = vec![];
        for line in reader.lines() {
            let txt = line?;
            if txt.contains('|') {
                let nums = txt
                    .split('|')
                    .map(|x| x.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<_>>();
                let before = nums.first().unwrap_or(&0);
                let after = nums.last().unwrap_or(&0);

                rules.entry(*before).or_default().push(*after);
            } else if txt.contains(',') {
                updates.push(
                    txt.split(',')
                        .map(|x| x.parse::<i32>().unwrap_or(0))
                        .collect::<Vec<_>>(),
                );
            }
        }

        Ok(Self { rules, updates })
    }

    fn is_sorted(&self, update: &[i32]) -> bool {
        let mut visited = HashSet::new();
        for page in update {
            for prev in visited.iter() {
                if let Some(after) = &self.rules.get(prev) {
                    if !after.iter().any(|a| a == page) {
                        return false;
                    }
                }
            }

            visited.insert(*page);
        }

        true
    }

    pub fn sum_of_correct_ordered(&self) -> i32 {
        self.updates
            .iter()
            .filter(|u| self.is_sorted(u))
            .map(|u| {
                let middle = ((u.len() + 1) / 2) - 1;

                u.get(middle).unwrap_or(&0)
            })
            .sum()
    }

    fn topological_sort(&self, update: &[i32]) -> Vec<i32> {
        let mut graph: HashMap<&i32, Vec<i32>> = HashMap::new();
        let mut level_tracker: HashMap<&i32, usize> = HashMap::new();

        for page in update {
            graph.insert(page, vec![]);
            level_tracker.insert(page, 0);
        }

        for (before, pages_after) in &self.rules {
            if update.contains(before) {
                for after in pages_after {
                    if update.contains(after) {
                        graph.entry(before).or_default().push(*after);
                        *level_tracker.entry(after).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<&i32> = level_tracker
            .iter()
            .filter(|&(_, &level)| level == 0)
            .map(|(node, _)| *node)
            .collect();

        let mut sorted_order = vec![];
        while let Some(current) = queue.pop_front() {
            sorted_order.push(*current);
            if let Some(neighbors) = graph.get(current) {
                for neighbor in neighbors {
                    if let Some(level) = level_tracker.get_mut(&neighbor) {
                        *level -= 1;
                        if *level == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        sorted_order
    }

    pub fn sum_of_after_topological_sort(&self) -> i32 {
        let mut total = 0;
        self.updates
            .iter()
            .filter(|u| !self.is_sorted(u))
            .for_each(|u| {
                let sorted = self.topological_sort(u);
                let middle = ((sorted.len() + 1) / 2) - 1;
                total += sorted.get(middle).unwrap_or(&0);
            });

        total
    }
}

#[cfg(test)]
mod test {
    use super::ElfPrinter;

    #[test]
    fn test_5_1() {
        let printer = ElfPrinter::new_from_data().unwrap();

        assert_eq!(4569, printer.sum_of_correct_ordered())
    }

    #[test]
    fn test_5_2() {
        let printer = ElfPrinter::new_from_data().unwrap();

        assert_eq!(6456, printer.sum_of_after_topological_sort())
    }
}
