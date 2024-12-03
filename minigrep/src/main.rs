// On top of the requirement from the rust programming language book
// we will add a few grep fonctionnality. As a reminder here is grep usage
// Usage: minigrep [OPTION]... PATTERNS [FILE]...
// Unlike grep we won't do options here
// but we will implement reading from stdin like grep
// and we will also handle multi-file pattern
// it would also be nice to have a usage
// (Maybe if the mood is there we'll do --help option at the end)

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pattern = args.get(1).expect("This is where the usage will go");
    let path_list = &args[2..];

    if path_list.len() > 0 {
        println!("Searching for \"{pattern}\" in files {path_list:?}");
    } else {
        println!("In the end this will search in stdin");
    }
}
