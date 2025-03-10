fn main() {
    let dic = std::env::current_dir().expect("Can't read current dirtionary.");
    
    println!("Current dictionary: {:#?}", dic);
}