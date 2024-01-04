pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        while i < chars.len() {
            let current: char = chars[i];
            let mut next: char = ' ';
            if i + 1 < chars.len() {
                next = chars[i + 1];
            }

            match current {
                'I' => match next {
                    'V' => {
                        result += 4;
                        i += 2;
                    }
                    'X' => {
                        result += 9;
                        i += 2;
                    }
                    _ => {
                        result += 1;
                        i += 1;
                    }
                },
                'V' => {
                    result += 5;
                    i += 1;
                }
                'X' => match next {
                    'L' => {
                        result += 40;
                        i += 2;
                    }
                    'C' => {
                        result += 90;
                        i += 2;
                    }
                    _ => {
                        result += 10;
                        i += 1;
                    }
                },
                'L' => {
                    result += 50;
                    i += 1;
                }
                'C' => match next {
                    'D' => {
                        result += 400;
                        i += 2;
                    }
                    'M' => {
                        result += 900;
                        i += 2;
                    }
                    _ => {
                        result += 100;
                        i += 1;
                    }
                },
                'D' => {
                    result += 500;
                    i += 1;
                }
                'M' => {
                    result += 1000;
                    i += 1;
                }
                _ => {
                    panic!("Cannot")
                }
            }
        }
        return result;
    }
}
