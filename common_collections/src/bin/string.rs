const INITIAL_DATA: &str = "Initial data";

fn with_push_str(str: &mut String, s: &str) {
    str.push_str(s);
}

fn with_push(str: &mut String, c: char) {
    str.push(c);
}

fn with_plus_operator(str: String, s: &str) -> String {
    str + s
}

fn with_format_macro(s1: &str, s2: &str, s3: &str) -> String {
    format!("-{s1}-{s2}-{s3}-")
}

fn main() {
    let some_string = INITIAL_DATA.to_string();
    let some_other_string = String::from(INITIAL_DATA);
    println!("{some_string} is the same as {some_other_string}");
    let mut base = String::new();
    with_push_str(&mut base, "-Added with push-");
    with_push(&mut base, '_');
    let not_a_base_anymore = with_plus_operator(base, "-added with + operator");
    let _concat_enough = with_format_macro("tic", &not_a_base_anymore, "toe");
    let hello = String::from("Здравствуйте");
    println!("str: \"{hello}\" has len: {}", hello.len());
    for b in hello.bytes() {
        println!("b\'{b}\'");
    }
    for c in hello.chars() {
        println!("c\'{c}\'");
    }
}
