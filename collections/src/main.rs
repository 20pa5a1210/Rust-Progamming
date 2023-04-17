fn main() {
    // let v:Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);  

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    let third : Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }




}
