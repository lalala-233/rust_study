fn main() {
    let str = String::from("hello world!");
    println!("{}", first_word(&str));
    let _s = "Hello world";
    println!("{}", &"123"[1..2]);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
