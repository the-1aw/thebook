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

fn dont_take_ownership(str: &String) -> usize {
    str.len()
}

fn mutable_borrowing(str: &mut String) {
    str.push_str("+sneaky+");
}

// For all the first word variant we must asume str is ascii based
fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();
    let mut len = str.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            len = i;
            break;
        }
    }
    return &str[..len];
}

fn first_word_no_mut(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}

fn first_word_no_mut_good_signature(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}

fn main() {
    let s1 = String::from("hello");
    take_ownership(s1);

    let s2 = give_ownership();

    let new_s2 = take_then_give_back(s2);
    let new_s2 = take_then_give_back(new_s2);
    let (len, str) = is_this_go_or_rust(new_s2);
    println!("str contains \"{str}\" and has len: {len}");
    let len = dont_take_ownership(&str);
    println!("can still use \"{str}\" and it still has len: {len}");
    let mut s3 = String::from("borrow me");
    mutable_borrowing(&mut s3);
    println!("value from s3 is now: {s3}");
    println!("the first word is {}", first_word(&String::from("one two")));
    println!(
        "the first word is also {}",
        first_word_no_mut(&String::from("one two"))
    );
    println!(
        "the first word is still {}",
        first_word_no_mut_good_signature("one two")
    );
}
