use std::collections::HashMap;

pub fn test() {
    println!("II: {}", roman_to_int("II".to_string()));
    println!("XVI: {}", roman_to_int("XVI".to_string()));
    println!("MCMXCIV: {}", roman_to_int("MCMXCIV".to_string()));
}

/*pub fn roman_to_int_try_2(s: String) -> i32 {
    let chars = s.chars();
    let map: HashMap<&str, i32> = [
        ("I", 1), ("V", 5), ("X", 10), ("L", 50), 
        ("C", 100), ("D", 500), ("M", 1000)
    ]
    .iter()
    .cloned()
    .collect();

    let mut int = 0;

    let mut last_ele = ' ';
    for ele in chars {
        if ele != 'I' {
            let ele_before = 
        }
    }

    int
}*/

pub fn roman_to_int(s: String) -> i32 {
    let map = create_map();
    let mut int = 0;
    let mut buf = s;
    let mappings = [("IV", 4), ("IX", 9), ("XL", 40), ("XC", 90), ("CD", 400), ("CM", 900)];
    for &(roman, value) in &mappings {
        if buf.contains(roman) {
            buf = buf.replace(roman, "");
            int += value;
        }
    }

    for ele in buf.chars() {
        let num = map
            .get(ele.to_string().as_str())
            .expect("Unallowed character");
        int += num;
    }

    int
}

fn create_map<'a>() -> HashMap<&'a str, i32> {
    let mut map = HashMap::with_capacity(7);
    map.insert("I", 1);
    map.insert("V", 5);
    map.insert("X", 10);
    map.insert("L", 50);
    map.insert("C", 100);
    map.insert("D", 500);
    map.insert("M", 1000);
    map
}
