fn main() {
    println!("Hello, world!");
    let years = vec![2015, 2016, 2017, 2018, 2019];
    // let years1 = print_years(years);
    // println!("years1: {:?}", years1);
    print_years(years.clone(), "*"); // <-- years is borrowed here
    print_years(years, "#"); // <-- years is borrowed here
}

fn print_years(years: Vec<i32>, label: &str) {
    for year in years.iter() {
        println!("{} {}", label, year);
    }
    // return years; // <-- years is returned here
} // <-- years is dropped here
