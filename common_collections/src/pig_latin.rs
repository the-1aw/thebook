// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)

const VOWELS: &[u8] = b"aeiouAEIOU";

enum PigType {
    Consonant,
    Vowel,
    NonPiggable,
}

fn pig_consonant(str: &str) -> Option<String> {
    let buffer: &[u8] = str.as_bytes();
    match (buffer.get(0), buffer.get(1..)) {
        (Some(consonant), Some(buffer)) => match std::str::from_utf8(buffer) {
            Ok(str) => Some(format!("{}-{}ay", str, consonant.escape_ascii())),
            _ => None,
        },
        _ => None,
    }
}

fn pig_vowel(str: &str) -> String {
    format!("{str}-hay")
}

fn get_pig_type(str: &str) -> PigType {
    let first_byte = str.bytes().nth(0);
    match first_byte {
        Some(first_byte @ b'a'..=b'z') | Some(first_byte @ b'A'..=b'Z') => {
            if VOWELS.contains(&first_byte) {
                PigType::Vowel
            } else {
                PigType::Consonant
            }
        }
        _ => PigType::NonPiggable,
    }
}

pub fn to_pig_latin(str: &str) -> Option<String> {
    match get_pig_type(str) {
        PigType::Vowel => Some(pig_vowel(str)),
        PigType::Consonant => pig_consonant(str),
        PigType::NonPiggable => {
            println!("\"{str}\" is not a piggable string");
            None
        }
    }
}
