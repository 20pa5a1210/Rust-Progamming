fn main() {
    println!("Hello, world!");
    let years = vec![2015, 2016, 2017, 2018, 2019];
    // let years1 = print_years(years);
    // println!("years1: {:?}", years1);
    print_years(years.clone(), "*"); // <-- years is borrowed here
    print_years(years, "#"); // <-- years is borrowed here
    palindrome("aaa");
    loops();
    collection();
}

fn print_years(years: Vec<i32>, label: &str) {
    for year in years.iter() {
        println!("{} {}", label, year);
    }
    // return years; // <-- years is returned here
} // <-- years is dropped here

fn palindrome(word: &str) -> bool {
    let mut chars = word.chars();
    loop {
        match (chars.next(), chars.next_back()) {
            (Some(x), Some(y)) if x == y => (),
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };
    println!("The result is {}", result);
}

fn collection() {
    let mut index = 0;
    let arr = [1, 2, 3, 4, 5];
    while index < arr.len() {
        println!("{}", arr[index]);
        index += 1;
    }
}

