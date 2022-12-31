pub fn inspect (arg: &String) {
    if arg.ends_with("s") {
        println!("plural");
    } else {
        println!("singular");
    }
}

pub fn change (arg: &mut String) -> &String {
    if !arg.ends_with("s") {
        arg.push_str("s");
    }
    arg
}

pub fn eat (arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn bedazzle (arg: &mut String) {
    *arg = String::from("sparkly");
}