pub mod l125_is_palindrome;
pub mod l13_roman_to_int;
pub mod l14_longest_common_prefix;
pub mod l1_two_sum;

fn main() {
    let lesson = 1;
    match lesson {
        1 => {
            let data = vec![1, 2, 3, 4, 5];
            let target = 7;
            println!(
                "For {}, target {}",
                data.clone()
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<String>(),
                target
            );
            println!(
                "Result is: {}",
                l1_two_sum::Solution::two_sum(data.clone(), target)
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<String>()
            );
        }
        13 => {
            let my_str = String::from("XIV");
            println!("Roman {}", my_str);
            println!("Int {}", l13_roman_to_int::Solution::roman_to_int(my_str));
        }
        14 => {
            let strs = vec![String::from("ab"), String::from("a")];
            println!(
                "{}",
                l14_longest_common_prefix::Solution::longest_common_prefix(strs)
            );
        }
        125 => {
            println!(
                "aTrt_a is_palindrome? {}",
                l125_is_palindrome::Solution::is_palindrome(String::from("aTrt_a"))
            );
            println!(
                "aTrt_b is_palindrome? {}",
                l125_is_palindrome::Solution::is_palindrome(String::from("aTrt_b"))
            );
            println!(
                "a is_palindrome? {}",
                l125_is_palindrome::Solution::is_palindrome(String::from("a"))
            );
        }
        _ => {
            println!("TODO");
        }
    }
}
