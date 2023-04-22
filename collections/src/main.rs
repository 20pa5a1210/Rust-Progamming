fn main() {
    // let v:Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let first = &v[0];
    // v.push(6);
    println!("The first element is: {}", first);

    let mut v = vec![100, 32, 57, 89, 12, 45, 67, 90];

    for i in &mut v {
        println!("{}", i);
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    impl SpreadsheetCell {
        fn value(&self) -> i32 {
            match self {
                SpreadsheetCell::Int(i) => *i,
                SpreadsheetCell::Float(f) => *f as i32,
                SpreadsheetCell::Text(s) => s.len() as i32,
            }
        }
    }

    println!("{}", row[0].value());
    println!("{}", row[1].value());
    {
        let v = vec![100, 32, 57];
    }
    
}
