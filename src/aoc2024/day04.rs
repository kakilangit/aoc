use std::{borrow::Cow, collections::HashMap, io::BufRead};

use crate::read_input;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }
}

#[derive(Default)]
pub struct ElfMonitor {
    matrix: Vec<Vec<Cow<'static, str>>>,
    from_right: Vec<Cow<'static, str>>,
    from_left: Vec<Cow<'static, str>>,
    from_top: Vec<Cow<'static, str>>,
    from_bottom: Vec<Cow<'static, str>>,
    diagonal_top_right: Vec<Cow<'static, str>>,
    diagonal_top_left: Vec<Cow<'static, str>>,
    diagonal_bottom_left: Vec<Cow<'static, str>>,
    diagonal_bottom_right: Vec<Cow<'static, str>>,
}

impl ElfMonitor {
    pub fn new_from_data() -> anyhow::Result<Self> {
        let reader = read_input("2024/4.txt")?;

        let matrix: Vec<Vec<Cow<'static, str>>> = reader
            .lines()
            .map(|line| line.map(|l| l.chars().map(|c| Cow::from(c.to_string())).collect()))
            .collect::<Result<_, _>>()?;

        let mut monitor = Self {
            matrix,
            ..Default::default()
        };
        monitor.calculate_vectors();

        Ok(monitor)
    }

    fn calculate_vectors(&mut self) {
        self.calculate_straights();
        self.calculate_diagonals();
    }

    pub fn count_xmas(&self) -> anyhow::Result<i32> {
        // let mut trie = Trie::new();
        // trie.insert("XMAS");
        //
        let combined: Vec<Cow<'static, str>> = self
            .from_left
            .iter()
            .chain(self.from_right.iter())
            .chain(self.from_top.iter())
            .chain(self.from_bottom.iter())
            .chain(self.diagonal_top_left.iter())
            .chain(self.diagonal_top_right.iter())
            .chain(self.diagonal_bottom_left.iter())
            .chain(self.diagonal_bottom_right.iter())
            .filter(|x| x.len() >= 4)
            .cloned()
            .collect();

        Ok(combined
            .iter()
            .map(|word| Self::brute_count_xmas(word))
            .sum::<i32>())
    }

    fn brute_count_xmas(word: &str) -> i32 {
        let mut total = 0;
        let mut chars = word.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == 'X' && chars.peek() == Some(&'M') {
                chars.next();
                if chars.peek() == Some(&'A') {
                    chars.next();
                    if chars.peek() == Some(&'S') {
                        chars.next();
                        total += 1;
                    }
                }
            }
        }

        total
    }

    fn calculate_straights(&mut self) {
        self.from_left = self
            .matrix
            .iter()
            .map(|row| Cow::from(row.concat()))
            .collect();
        self.from_right = self
            .matrix
            .iter()
            .map(|row| Cow::from(row.iter().rev().cloned().collect::<String>()))
            .collect();

        let num_cols = self.matrix[0].len();
        self.from_top = (0..num_cols)
            .map(|col| {
                Cow::from(
                    self.matrix
                        .iter()
                        .map(|row| &row[col])
                        .cloned()
                        .collect::<String>(),
                )
            })
            .collect();

        self.from_bottom = (0..num_cols)
            .map(|col| {
                Cow::from(
                    self.matrix
                        .iter()
                        .rev()
                        .map(|row| &row[col])
                        .cloned()
                        .collect::<String>(),
                )
            })
            .collect();
    }

    fn calculate_diagonals(&mut self) {
        let n = self.matrix.len();

        let get_diagonal =
            |start_row: i32, start_col: i32, row_step: i32, col_step: i32, reverse: bool| {
                let mut diagonal: Vec<Cow<'static, str>> = vec![];
                let mut row = start_row;
                let mut col = start_col;
                let mut current_diagonal = String::new();

                while row >= 0 && row < n as i32 && col >= 0 && col < n as i32 {
                    current_diagonal.push_str(&self.matrix[row as usize][col as usize]);
                    row += row_step;
                    col += col_step;
                }

                if reverse {
                    current_diagonal = current_diagonal.chars().rev().collect();
                }

                diagonal.push(Cow::from(current_diagonal));
                diagonal
            };

        for i in 0..n {
            let diag1 = get_diagonal(n as i32 - 1 - i as i32, 0, -1, 1, false);
            let diag2 = get_diagonal(n as i32 - 1, i as i32, -1, 1, false);
            self.diagonal_bottom_left.extend(diag1);
            self.diagonal_bottom_left.extend(diag2);
        }
        self.diagonal_bottom_left.dedup();

        for i in 0..n {
            let diag1 = get_diagonal(n as i32 - 1 - i as i32, n as i32 - 1, -1, -1, false);
            let diag2 = get_diagonal(n as i32 - 1, n as i32 - 1 - i as i32, -1, -1, false);
            self.diagonal_bottom_right.extend(diag1);
            self.diagonal_bottom_right.extend(diag2);
        }
        self.diagonal_bottom_right.dedup();

        for i in 0..n {
            let diag1 = get_diagonal(n as i32 - 1 - i as i32, 0, -1, 1, true);
            let diag2 = get_diagonal(n as i32 - 1, i as i32, -1, 1, true);
            self.diagonal_top_left.extend(diag1);
            self.diagonal_top_left.extend(diag2);
        }
        self.diagonal_top_left.dedup();

        for i in 0..n {
            let diag1 = get_diagonal(n as i32 - 1 - i as i32, n as i32 - 1, -1, -1, true);
            let diag2 = get_diagonal(n as i32 - 1, n as i32 - 1 - i as i32, -1, -1, true);
            self.diagonal_top_right.extend(diag1);
            self.diagonal_top_right.extend(diag2);
        }
        self.diagonal_top_right.dedup();
    }
}

#[cfg(test)]
mod test {
    use super::ElfMonitor;

    #[test]
    fn test_4_1() {
        let monitor = ElfMonitor::new_from_data().unwrap();

        let total = monitor.count_xmas().unwrap();
        assert_eq!(2562, total);
    }
}
