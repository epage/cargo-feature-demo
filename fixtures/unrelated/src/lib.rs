pub fn print() {
    println!("unrelated");
    #[cfg(feature = "feature_name")]
    println!("unrelated feature_name");
}
