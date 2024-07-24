pub fn print(parent: &str) {
    let me = env!("CARGO_PKG_NAME");
    println!("{parent}->{me}");
    #[cfg(feature = "feature_name")]
    println!("{parent}->{me} feature_name");
}
