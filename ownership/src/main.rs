fn main() {
    let mut s = String::from("hello");
    let mut s2 = String::from(", world!");

    println!("{}", s);

    s = takes_ownership2(s);

    println!("{}", s);

    let res = takes_ownership3(s, s2);
    s = res.0;
    s2 = res.1;

    println!("{} {}", s, s2);

    let (s, s2) = takes_ownership3(s, s2);

    println!("{} {}", s, s2);

    takes_ownership4(&s);

    println!("{}", s);

    let mut s3 = s.clone();
    s3.push_str(&s2);

    let first_word = first_word(&s3);

    println!("first word = {}", first_word);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership2(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn takes_ownership3(some_string1: String, some_string2: String) -> (String, String) {
    println!("{} {}", some_string1, some_string2);
    (some_string1, some_string2)
}

fn takes_ownership4(some_string: &String) {
    println!("{}", some_string);
}

fn first_word(s: &str) -> &str {
    let i = match s.find(" ") {
        Option::None => 0,
        Some(num) => num,
    };

    &s[0..i]
}
