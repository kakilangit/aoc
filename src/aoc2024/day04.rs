use std::{borrow::Cow, io::BufRead};

use crate::read_input;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    DiagUpRight,
    DiagUpLeft,
    DiagDownRight,
    DiagDownLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    // 0.0 is top left
    fn matrix_path(
        &self,
        n: i32,
        direction: Direction,
        matrix_size: (i32, i32),
    ) -> anyhow::Result<Vec<Self>> {
        if n < 1 {
            anyhow::bail!("negative is forbidden, use direction to move")
        }

        let mut path = vec![];

        match direction {
            Direction::Right => {
                if self.0 + n > (matrix_size.0 - 1) {
                    anyhow::bail!("out of bounds")
                }

                for i in 0..n {
                    path.push(Pos(self.0 + i + 1, self.1));
                }
            }
            Direction::Left => {
                if self.0 - n < 0 {
                    anyhow::bail!("out of bounds")
                }

                for i in 0..n {
                    path.push(Pos(self.0 - (i + 1), self.1));
                }
            }
            Direction::Up => {
                if self.1 - n < 0 {
                    anyhow::bail!("out of bounds")
                }

                for i in 0..n {
                    path.push(Pos(self.0, self.1 - (i + 1)));
                }
            }
            Direction::Down => {
                if self.1 + n > (matrix_size.1 - 1) {
                    anyhow::bail!("out of bounds")
                }

                for i in 0..n {
                    path.push(Pos(self.0, self.1 + (i + 1)));
                }
            }
            Direction::DiagUpLeft => {
                for i in 0..n {
                    let delta = i + 1;
                    let x = self.0 - delta;
                    let y = self.1 - delta;

                    if x < 0 || y < 0 {
                        anyhow::bail!("out of bounds")
                    }

                    path.push(Pos(x, y));
                }
            }
            Direction::DiagUpRight => {
                for i in 0..n {
                    let delta = i + 1;
                    let x = self.0 + delta;
                    let y = self.1 - delta;

                    if x > matrix_size.0 - 1 || y < 0 {
                        anyhow::bail!("out of bounds")
                    }

                    path.push(Pos(x, y));
                }
            }
            Direction::DiagDownLeft => {
                for i in 0..n {
                    let delta = i + 1;
                    let x = self.0 - delta;
                    let y = self.1 + delta;

                    if x < 0 || y > matrix_size.1 - 1 {
                        anyhow::bail!("out of bounds")
                    }

                    path.push(Pos(x, y));
                }
            }
            Direction::DiagDownRight => {
                for i in 0..n {
                    let delta = i + 1;
                    let x = self.0 + delta;
                    let y = self.1 + delta;

                    if x > matrix_size.0 - 1 || y > matrix_size.1 - 1 {
                        anyhow::bail!("out of bounds")
                    }

                    path.push(Pos(x, y));
                }
            }
        }

        Ok(path)
    }
}

#[derive(Default)]
pub struct ElfMonitor {
    matrix: Vec<Vec<Cow<'static, str>>>,
}

impl ElfMonitor {
    pub fn new_from_data() -> anyhow::Result<Self> {
        let reader = read_input("2024/4.txt")?;

        let matrix: Vec<Vec<Cow<'static, str>>> = reader
            .lines()
            .map(|line| line.map(|l| l.chars().map(|c| Cow::from(c.to_string())).collect()))
            .collect::<Result<_, _>>()?;

        Ok(Self { matrix })
    }

    fn find_mas(&self, start: Pos) -> usize {
        let paths = self.mas_paths_in_all_direction(start);
        paths
            .iter()
            .filter(|p| self.matrix_string(p) == "MAS")
            .count()
    }

    fn matrix_string(&self, paths: &[Pos]) -> Cow<'static, str> {
        paths
            .iter()
            .map(|p| {
                let x = p.0 as usize;
                let y = p.1 as usize;

                self.matrix[x][y].chars().as_str()
            })
            .collect()
    }

    fn mas_paths_in_all_direction(&self, start: Pos) -> Vec<Vec<Pos>> {
        let size = (self.matrix[0].len() as i32, self.matrix.len() as i32);
        [
            Direction::Right,
            Direction::Left,
            Direction::Up,
            Direction::Down,
            Direction::DiagUpLeft,
            Direction::DiagUpRight,
            Direction::DiagDownLeft,
            Direction::DiagDownRight,
        ]
        .iter()
        .filter_map(|dir| {
            if let Ok(path) = start.matrix_path(3, *dir, size) {
                Some(path)
            } else {
                None
            }
        })
        .collect()
    }

    pub fn count_xmas(&self) -> i32 {
        let mut total = 0;
        let mut pos = Pos(0, 0);

        let max_down = self.matrix.len() as i32 - 1;
        let max_right = self.matrix[0].len() as i32 - 1;

        while pos.0 <= max_down && pos.1 <= max_right {
            let c = &self.matrix[pos.0 as usize][pos.1 as usize];
            if c == "X" {
                total += self.find_mas(pos);
            }

            if pos.1 < max_right {
                pos.1 += 1;
            } else {
                pos.1 = 0;
                pos.0 += 1;
            }
        }

        total as i32
    }

    pub fn count_crossmas(&self) -> i32 {
        let mut total = 0;
        let mut pos = (1, 1);

        let max_down = self.matrix.len() - 2;
        let max_right = self.matrix[0].len() - 2;

        while pos.0 <= max_down && pos.1 <= max_right {
            let c = &self.matrix[pos.0][pos.1];
            if c == "A" {
                let top_left = &self.matrix[pos.0 - 1][pos.1 - 1];
                let bottom_right = &self.matrix[pos.0 + 1][pos.1 + 1];
                let top_right = &self.matrix[pos.0 - 1][pos.1 + 1];
                let bottom_left = &self.matrix[pos.0 + 1][pos.1 - 1];

                if Self::valid_cross(top_left, top_right, bottom_left, bottom_right) {
                    total += 1;
                }
            }

            if pos.1 < max_right {
                pos.1 += 1;
            } else {
                pos.1 = 1;
                pos.0 += 1;
            }
        }

        total
    }

    fn valid_cross(top_left: &str, top_right: &str, bottom_left: &str, bottom_right: &str) -> bool {
        let valid_char = |s| matches!(s, "M" | "S");

        if !valid_char(top_left)
            || !valid_char(top_right)
            || !valid_char(bottom_left)
            || !valid_char(bottom_right)
        {
            return false;
        }

        if top_left == bottom_right || top_right == bottom_left {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_4_1() {
        let monitor = ElfMonitor::new_from_data().unwrap();

        let total = monitor.count_xmas();
        assert_eq!(2562, total);
    }

    #[test]
    fn test_4_2() {
        let monitor = ElfMonitor::new_from_data().unwrap();

        let total = monitor.count_crossmas();
        assert_eq!(1902, total);
    }

    #[test]
    fn test_path() {
        let pos = Pos(5, 5);
        let size = (10, 10);

        struct Tc {
            n: i32,
            dir: Direction,
            expected: Vec<Pos>,
            msg: &'static str,
        }
        let tcs = [
            Tc {
                n: 1,
                dir: Direction::Right,
                expected: vec![Pos(6, 5)],
                msg: "right",
            },
            Tc {
                n: 1,
                dir: Direction::Up,
                expected: vec![Pos(5, 4)],
                msg: "top",
            },
            Tc {
                n: 4,
                dir: Direction::Down,
                expected: vec![Pos(5, 6), Pos(5, 7), Pos(5, 8), Pos(5, 9)],
                msg: "down",
            },
            Tc {
                n: 2,
                dir: Direction::DiagUpLeft,
                expected: vec![Pos(4, 4), Pos(3, 3)],
                msg: "diagonal top left",
            },
            Tc {
                n: 2,
                dir: Direction::DiagUpRight,
                expected: vec![Pos(6, 4), Pos(7, 3)],
                msg: "diagonal top right",
            },
            Tc {
                n: 2,
                dir: Direction::DiagDownLeft,
                expected: vec![Pos(4, 6), Pos(3, 7)],
                msg: "diagonal bottom left",
            },
            Tc {
                n: 2,
                dir: Direction::DiagDownRight,
                expected: vec![Pos(6, 6), Pos(7, 7)],
                msg: "diagonal bottom right",
            },
        ];

        for t in tcs {
            let actual = pos.matrix_path(t.n, t.dir, size).unwrap();
            assert_eq!(t.expected, actual, "{}", t.msg);
        }
    }
}
