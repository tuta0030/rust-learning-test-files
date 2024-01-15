enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn main () {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}