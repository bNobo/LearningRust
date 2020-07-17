fn main() {
    let res = add(2, 2);

    println!("res = {0}", res);

    let res = substract(10, 3);

    println!("res = {0}", res);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {} and counter = {}", result, counter);

    let counter = 0;

    let result = counter_loop(counter);

    println!("result = {} and counter = {}", result, counter);
}

fn counter_loop(mut counter: i32) -> i32 {
    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    counter * 2
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn substract(x: i32, y: i32) -> i32 {
    return x - y;
}
