use std::io;

pub fn get_input(msg: String) -> String {
    println!("{} ", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().is_empty() {
        input = "0".to_string();
    }
    input
}

pub fn get_input_f64(msg: String) -> f64 {
    let mut input: f64 = get_input(msg).trim().parse().unwrap();

    while input.is_nan() {
        input = get_input("Enter a valid input: ".to_string())
            .trim()
            .parse()
            .unwrap();
    }

    while input.is_sign_negative() {
        input = get_input("Enter a non-negative input: ".to_string())
            .trim()
            .parse()
            .unwrap();
    }

    input
}

pub fn get_input_i32(msg: String) -> i32 {
    let mut input: i32 = get_input(msg).trim().parse().unwrap();

    while input.is_negative() {
        input = get_input("Enter a non-negative input: ".to_string())
            .trim()
            .parse()
            .unwrap();
    }

    input
}

pub fn write_to_file(filename: &str, data: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write_all(data.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
