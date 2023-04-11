fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{}", word);
    s.clear();
    let s = String::from("Hello world");
    let hello = &s[..];
    println!("{}", hello);
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", assert_eq!(slice, &[2, 3]));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
