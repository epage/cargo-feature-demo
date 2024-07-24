fn main() {
    let me = env!("CARGO_PKG_NAME");
    println!("{me}");
    #[cfg(feature = "a")]
    println!("{me} a");
    #[cfg(feature = "a")]
    dep_name::print(me);
}
