fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    // read elements
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    v.push(4);
    v.push(5);

    inference();

    // iterating over values and modify them
    for i in &mut v {
        *i += 50;
    }

    // iterating over values (read-only)
    for i in v {
        println!("{}", i);
    }

    // can't access v anymore here because of previous iteration taking ownership on v (use &v above to borrow variable and prevent this)
    //let fist = &v[0];

    // String
    let s = "initial_contents".to_string();
    let s = String::from("initial_contents");
    // Choosing between the two above is a matter of taste

    let hello = String::from("こんにちは");
    let hello = String::from("你好");

    // Appending
    let mut s = String::from("foo");
    s.push_str("bar");

    // Concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s = s1 + "-" + &s2 + "-" + &s3;
    // format does not take ownership on s1, unlike "+" concat operator which rely on the "add" method
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    // Iterating over String
    for c in "Hello".chars() {
        println!("{}", c);
    }

    // HashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Overwrite Blue key with a new value
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let score = scores.get("Blue");

    match score {
        Some(score) => println!("{}", score),
        None => println!("no score"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn inference() {
    // Rust can infer type from following code, even in another function !
    let mut v = Vec::new();

    v.push(1);

    push_value(&mut v, 2);

    v.push(3);

    println!("{:?}", v);
}

fn push_value(v: &mut Vec<i32>, n: i32) {
    v.push(n);
}
