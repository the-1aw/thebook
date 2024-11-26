enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let vector: Vec<usize> = Vec::new();
    dbg!(vector);
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    let first: &usize = &vector[0];
    println!("First element is {}", first);
    let first: Option<&usize> = vector.get(0);
    match first {
        Some(first) => println!("First element is {}", first),
        _ => println!("There is no first element"),
    }
    let tenth: Option<&usize> = vector.get(10);
    match tenth {
        Some(tenth) => println!("Tenth element is {}", tenth),
        _ => println!("There is no tenth element"),
    }
    for i in &vector {
        println!("{i}");
    }
    let spreadsheet_row = vec![
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Text(String::from("some sort of text")),
        SpreadsheetCell::Float(1.0),
    ];
    for i in &spreadsheet_row {
        match i {
            SpreadsheetCell::Int(num) => println!("int: {}", num),
            SpreadsheetCell::Text(txt) => println!("txt: {txt}"),
            SpreadsheetCell::Float(num) => println!("float: {num}"),
        }
    }
}
