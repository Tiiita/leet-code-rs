fn main() {
    palindrome(12321); //Should print yes
    palindrome(1234); //Should print 
    palindrome(19191); //Should print yes again
}

fn palindrome(num: i32) {
    let binding = num.to_string();
    let as_chars: Vec<char> = binding.chars().into_iter().collect();

    for (i, ele) in  as_chars.iter().enumerate() {
        if let Some(char_end) = as_chars.get(as_chars.len() - i - 1) {
            if ele.eq_ignore_ascii_case(char_end) {
                continue;
            } else {
                println!("No palindrome!");
                return;
            }
        }
    }

    println!("Is a palindrome!");
}
