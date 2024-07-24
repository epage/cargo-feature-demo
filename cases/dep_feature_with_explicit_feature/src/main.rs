fn main() {
    let me = env!("CARGO_PKG_NAME");
    println!("{me}");
    #[cfg(feature = "a")]
    println!("{me} a");
    #[cfg(feature = "dep_name")]
    println!("{me} dep_name");
    #[cfg(feature = "dep_name")]
    unrelated::print(me);
}
