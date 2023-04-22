// String is a growable, mutable, owned, UTF-8 encoded string type
// String literals are of type &str
// String literals are immutable
// String literals are stored in the binary
// String literals are not null terminated
use std::collections::HashMap;

fn main() {
    let data = "initals: J. K. Rowling";
    println!("{}", data);
    let s = data.to_string();
    println!("{}", s);

    let mut s :String = String::from("foo");
    s.push_str("foo");
    println!("{}", s);

    let mut s = String::from("lo");
    let s2 = " ";
    s.push_str(s2);
    s.push('l');
    println!("s2 is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2.clone();// note s1 has been moved here and can no longer be used
    println!("{}", s3); // s3 is now the owner of the string
    println!("{}", s1); // s2 is still vali1


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{new}-{s2}-{s3}",new = s1); // format! does not take ownership of the string
    println!("{}", s);// s1 is still valid here

    for i in s.chars() {
        println!("{i}");
    }

    let mut scores = HashMap::new();
    scores.insert("j".to_string(), 10);
    scores.insert(String::from("k"), 10);
    scores.insert(String::from("k"), 100);

    // let score = scores.get("k").unwrap();
    // println!("{:?}", score);

    for i in scores{
        println!("{:?}", i);
    }
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for i in map{
        println!("{:?}", i);
    }

}
