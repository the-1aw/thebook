// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn challenge_employee_list() {}

use common_collections::integer_challenge as int_chal;
use common_collections::pig_latin;

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
    assert_eq!(pig_latin::to_pig_latin(""), None);
    assert_eq!(pig_latin::to_pig_latin("_"), None);
    assert_eq!(pig_latin::to_pig_latin("🧡"), None);
    assert_eq!(pig_latin::to_pig_latin("🧡💛💚💙💜"), None);
    assert_eq!(pig_latin::to_pig_latin("a"), Some(String::from("a-hay")));
    assert_eq!(pig_latin::to_pig_latin("A"), Some("A-hay".to_string()));
    assert_eq!(pig_latin::to_pig_latin("p"), Some("-pay".to_string()));
    assert_eq!(pig_latin::to_pig_latin("P"), Some("-Pay".to_string()));
    assert_eq!(
        pig_latin::to_pig_latin("Apple"),
        Some("Apple-hay".to_string())
    );
    assert_eq!(pig_latin::to_pig_latin("Pig"), Some("ig-Pay".to_string()));
    challenge_employee_list()
}
