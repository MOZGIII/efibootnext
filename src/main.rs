use efinextboot::load_options;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut manager = efivar::system();
    for load_option_result in load_options(manager.as_mut()) {
        let load_option = load_option_result?;
        println!("{:04X} {}", load_option.number, load_option.description);
    }
    Ok(())
}
