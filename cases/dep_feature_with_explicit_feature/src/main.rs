fn main() {
    #[cfg(feature = "a")]
    println!("a");
    #[cfg(feature = "dep_name")]
    println!("dep_name");
    #[cfg(feature = "dep_name")]
    unrelated::print();
}
