pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let new_s = transform_str_to_palindromable(&s);
        let new_s = new_s.as_bytes();
        let new_s_len = new_s.len();
        if new_s_len < 2 {
            return true;
        }
        for (idx, my_byte) in new_s.iter().enumerate() {
            if my_byte != &new_s[new_s_len - idx - 1] {
                return false;
            }

            if idx > (new_s_len - 1) / 2 {
                return true;
            }
        }
        false
    }
}

fn transform_str_to_palindromable(s: &String) -> String {
    return s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>();
}
