// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)
fn challenge_pig_lating() {}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn challenge_employee_list() {}

use common_collections::integer_challenge as int_chal;

fn main() {
    let integer_list_odd = [1, 9, 12, 43, 9, 3, 32];
    let integer_list_even = [8, 9, 43, 42, 9, 132];
    assert_eq!(
        vec![12, 9],
        int_chal::challenge_integer_list(&integer_list_odd)
    );
    assert_eq!(
        vec![42, 9],
        int_chal::challenge_integer_list(&integer_list_even)
    );
    challenge_pig_lating();
    challenge_employee_list()
}
