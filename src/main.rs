use efinextboot::{load_options, set_boot_next};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut manager = efivar::system();

    match std::env::args().skip(1).next() {
        None => {
            for load_option_result in load_options(manager.as_mut()) {
                let load_option = load_option_result?;
                println!("{:04X} {}", load_option.number, load_option.description);
            }
        }
        Some(boot_next_str) => {
            let boot_next: u16 = boot_next_str.parse()?;
            let _ = set_boot_next(manager.as_mut(), boot_next)?;
            println!("{:04X}", boot_next);
        }
    }
    Ok(())
}
