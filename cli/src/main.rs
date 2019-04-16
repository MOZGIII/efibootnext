#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use efibootnext::{load_options, set_boot_next};

mod boot_next_format;
use boot_next_format::BootNextFormat;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<std::error::Error>> {
    let mut manager = efivar::system();
    let default_boot_next_format: &str = &format!("{}", BootNextFormat::Hex);

    let matches = App::new("efibootnext")
        .version(crate_version!())
        .about("Controls BootNext EFI variable")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .aliases(&["ls", "dir"])
                .about("Prints possbile boot options"),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets the BootNext EFI variable")
                .arg(
                    Arg::with_name("boot_next")
                        .required(true)
                        .index(1)
                        .help("The value to set BootNext to"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .possible_values(&BootNextFormat::variants())
                        .default_value(default_boot_next_format)
                        .help("The format of the value"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => {
            for load_option_result in load_options(manager.as_mut()) {
                let load_option = load_option_result?;
                println!("{:04X} {}", load_option.number, load_option.description);
            }
            Ok(())
        }
        ("set", Some(submatches)) => {
            let format = value_t_or_exit!(submatches, "format", BootNextFormat);
            let boot_next: u16 = format
                .parse_boot_next(submatches, "boot_next")
                .unwrap_or_else(|e| e.exit());

            let _ = set_boot_next(manager.as_mut(), boot_next)?;

            println!("{:04X}", boot_next);
            Ok(())
        }
        _ => unreachable!(),
    }
}
