use std::{borrow::Cow, io::BufRead};

use anyhow::Ok;

use crate::read_input;

pub struct Computer {
    raw: Cow<'static, str>,
}

impl Computer {
    pub fn new_from_data() -> anyhow::Result<Self> {
        let reader = read_input("2024/3.txt")?;

        let mut raw = "".to_string();

        for line in reader.lines() {
            let s = line?;
            raw.push_str(&s);
        }

        Ok(Computer {
            raw: Cow::from(raw),
        })
    }

    fn find_mul(&self) -> anyhow::Result<Vec<(i32, i32)>> {
        let mut nums = vec![];
        let mut chars = self.raw.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == 'm' && chars.peek() == Some(&'u') {
                chars.next();
                if chars.peek() == Some(&'l') {
                    chars.next();
                    if chars.peek() == Some(&'(') {
                        chars.next();

                        let mut number1 = String::new();
                        while let Some(digit) = chars.peek() {
                            if digit.is_ascii_digit() {
                                number1.push(*digit);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        if chars.peek() == Some(&',') {
                            chars.next();

                            let mut number2 = String::new();
                            while let Some(digit) = chars.peek() {
                                if digit.is_ascii_digit() {
                                    number2.push(*digit);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }

                            if chars.peek() == Some(&')') {
                                chars.next();
                                nums.push((number1.parse::<i32>()?, number2.parse::<i32>()?));
                            }
                        }
                    }
                }
            }
        }

        Ok(nums)
    }

    fn find_mul_conditional(&self) -> anyhow::Result<Vec<(i32, i32)>> {
        let mut nums = vec![];
        let mut chars = self.raw.chars().peekable();
        let mut do_mul = true;
        while let Some(ch) = chars.next() {
            if ch == 'd' && chars.peek() == Some(&'o') {
                chars.next();
                if chars.peek() == Some(&'n') {
                    chars.next();
                    if chars.peek() == Some(&'\'') {
                        chars.next();
                        if chars.peek() == Some(&'t') {
                            chars.next();
                            do_mul = false;
                        }
                    }
                } else {
                    do_mul = true;
                }
            } else if ch == 'm' && chars.peek() == Some(&'u') {
                chars.next();
                if !do_mul {
                    continue;
                }

                if chars.peek() == Some(&'l') {
                    chars.next();
                    if chars.peek() == Some(&'(') {
                        chars.next();

                        let mut number1 = String::new();
                        while let Some(digit) = chars.peek() {
                            if digit.is_ascii_digit() {
                                number1.push(*digit);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        if chars.peek() == Some(&',') {
                            chars.next();

                            let mut number2 = String::new();
                            while let Some(digit) = chars.peek() {
                                if digit.is_ascii_digit() {
                                    number2.push(*digit);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }

                            if chars.peek() == Some(&')') {
                                chars.next();
                                nums.push((number1.parse::<i32>()?, number2.parse::<i32>()?));
                            }
                        }
                    }
                }
            }
        }

        Ok(nums)
    }

    pub fn do_multiplication(&self) -> anyhow::Result<i32> {
        let res = self.find_mul()?.iter().map(|(n1, n2)| (n1 * n2)).sum();
        Ok(res)
    }

    pub fn do_conditional_multiplication(&self) -> anyhow::Result<i32> {
        let res = self
            .find_mul_conditional()?
            .iter()
            .map(|(n1, n2)| (n1 * n2))
            .sum();
        Ok(res)
    }
}

fn part_one() -> anyhow::Result<i32> {
    let computer = Computer::new_from_data()?;
    computer.do_multiplication()
}

fn part_two() -> anyhow::Result<i32> {
    let computer = Computer::new_from_data()?;

    computer.do_conditional_multiplication()
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let ans = part_one().unwrap();
        assert_eq!(180233229, ans);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two().unwrap();
        assert_eq!(95411583, ans);
    }
}
