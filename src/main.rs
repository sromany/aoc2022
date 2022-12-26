fn main() -> std::io::Result<()> {
    println!("Testing values: {:?}", &"48-50,48-49".split(",").collect::<Vec<&str>>());
    Ok(())
}
