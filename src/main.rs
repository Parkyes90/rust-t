fn main() {
    let s1 = String::from("             hello");
    let len = first_word(&s1);
    println!("{}의 첫 단어는 {}입니다.", s1, len)
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
