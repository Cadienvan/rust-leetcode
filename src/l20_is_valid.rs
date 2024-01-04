use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut ongoing_brackets: Vec<char> = vec![];
        let openings: Vec<char> = vec!['(', '[', '{'];
        let closings: Vec<char> = vec![')', ']', '}'];
        let mut available_combinations: HashMap<char, char> = HashMap::new();
        available_combinations.insert(')', '(');
        available_combinations.insert(']', '[');
        available_combinations.insert('}', '{');

        for s_char in s.chars() {
            if openings.contains(&s_char) {
                ongoing_brackets.push(s_char);
            }

            if closings.contains(&s_char) {
                if ongoing_brackets.is_empty() {
                    return false;
                }

                if ongoing_brackets.pop().unwrap() != *available_combinations.get(&s_char).unwrap()
                {
                    return false;
                }
            }
        }

        ongoing_brackets.is_empty()
    }
}
