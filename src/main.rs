fn main() {
    let mut years = vec![2015, 2016, 2017, 2018, 2019];
    for year in years.iter_mut() {
        *year += 1;
    }

    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    for num in nums.iter() {
        println!("{}", num);
    }
    println!("{:?}", years);
    let months: Vec<String> = vec![
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ];
    println!("{:?}", months);
    let length: usize = months.len();
    println!("Length of months is {}", length);

    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];

    for num in nums {
        println!("{}", num);
    }
    increment_decrement(10);
} // End of main

fn increment_decrement(num: u8) {
    print_nums(num + 1, num - 1);
}

fn print_nums(num1: u8, num2: u8) {
    println!("num1 is {} and num2 is {}", num1, num2);
}

// Path: src/main.rs
