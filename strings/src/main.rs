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

}
