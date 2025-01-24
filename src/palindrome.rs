//Leet Code: https://leetcode.com/problems/palindrome-number/description/
//Level: Easy

use std::{char, str::Chars};

pub fn test() {
    palindrome_num(12321);
    palindrome("Rentner".chars());
    palindrome("test".chars());
    palindrome("😏💀d💀😏".chars());
}

fn palindrome_num(num: i32) {
    let bind = num.to_string();
    palindrome(bind.chars());
}

fn palindrome(chars: Chars<'_>) {
    let as_chars: Vec<char> = chars.clone().into_iter().collect();
    for (i, ele) in as_chars.iter().enumerate() {
        if let Some(char_end) = as_chars.get(as_chars.len() - i - 1) {
            if ele.eq_ignore_ascii_case(char_end) {
                continue;
            } else {
                println!("{} => No Palindrom! ❌", chars.as_str());
                return;
            }
        }
    }

    println!("{} => Is Palindrome! ✅", chars.as_str());
}