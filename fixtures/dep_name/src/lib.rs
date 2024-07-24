pub fn print() {
    println!("dep_name");
    #[cfg(feature = "feature_name")]
    println!("dep_name feature_name");
}
