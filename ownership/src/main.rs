fn take_ownership(str: String) {
    println!("{str}");
}

fn give_ownership() -> String {
    String::from("give ownership")
}

fn take_then_give_back(str: String) -> String {
    str
}

fn is_this_go_or_rust(s: String) -> (usize, String) {
    (s.len(), s)
}

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);

    let s2 = give_ownership();

    let new_s2 = take_then_give_back(s2);
    let new_s2 = take_then_give_back(new_s2);
    let (len, str) = is_this_go_or_rust(new_s2);
    println!("str contains \"{str}\" and has len: {len}");
}
