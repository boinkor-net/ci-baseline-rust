fn main() {
    // fails the "use .is_some() check"
    if let Some(_) = Some(1) {
        println!("yay");
    }
}
