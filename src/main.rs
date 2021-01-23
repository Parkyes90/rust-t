struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("parkyes90@gmail.com"),
        username: String::from("parkyes90"),
        active: true,
        sign_in_count: 1,
    };
    let s1 = String::from("             hello");
    let len = first_word(&s1);
    println!("{}의 첫 단어는 {}입니다.", s1, len);
    println!(
        "{},{},{},{}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );
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
