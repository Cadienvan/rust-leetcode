pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest = strs.first().unwrap().as_bytes();
        println!(
            "Longest: {}",
            std::str::from_utf8(longest).unwrap().to_string()
        );

        let mut iter = strs.iter();
        iter.next();

        for a_string in iter {
            if a_string.as_bytes().len() < longest.len() {
                longest = &longest[..a_string.as_bytes().len()];
            }
            for (idx, &current_byte) in a_string.as_bytes().iter().enumerate() {
                println!(
                    "Checking str {}, idx {}, so it means {}. Correspondance in longest: {}",
                    a_string,
                    idx,
                    current_byte.to_string(),
                    match longest.len() > idx {
                        true => longest[idx].to_string(),
                        false => String::from("OUT_OF_BOUNDS"),
                    }
                );
                if longest.len() <= idx {
                    break;
                }
                if longest[idx] != current_byte {
                    if idx == 0 {
                        return String::from("");
                    }
                    longest = &longest[..idx];
                    println!(
                        "New longest: {}",
                        std::str::from_utf8(longest).unwrap().to_string()
                    );
                    break;
                }
            }
        }

        return std::str::from_utf8(longest).unwrap().to_string();
    }
}
