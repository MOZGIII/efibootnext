//! Command line interface for operations on the EFI `BootNext` variable.

use clap::{Parser, Subcommand};
use efibootnext::Adapter;

/// The command line app invocation.
#[derive(Parser)]
#[command(name = "efibootnext")]
#[command(about = "Control BootNext EFI variable.", long_about = None)]
struct Invocation {
    /// The command that is invoked.
    #[command(subcommand)]
    command: Command,
}

/// CLI commands.
#[derive(Subcommand)]
enum Command {
    /// Prints possbile boot options.
    List,
    /// Print the given boot option description.
    Describe {
        /// The boot option to work with.
        load_option: String,
        /// The format to use when specifying the load option value.
        #[arg(short, long, env = "LOAD_OPTION_FORMAT", value_enum, default_value_t = LoadOptionFormat::Hex)]
        format: LoadOptionFormat,
    },
    /// Prints the value of the BootNext EFI variable.
    Get,
    /// Sets the BootNext EFI variable.
    Set {
        /// The value to set `BootNext` to.
        load_option: String,
        /// The format to use when specifying the load option value.
        #[arg(short, long, env = "LOAD_OPTION_FORMAT", value_enum, default_value_t = LoadOptionFormat::Hex)]
        format: LoadOptionFormat,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

/// Run the app and return the error.
fn run() -> Result<(), anyhow::Error> {
    let mut adapter = Adapter::default();

    let invocation = Invocation::parse();

    match invocation.command {
        Command::List => {
            for load_option_result in adapter.load_options()? {
                let load_option = load_option_result?;
                println!("{:04X} {}", load_option.number, load_option.description);
            }
            Ok(())
        }
        Command::Describe {
            format,
            load_option,
        } => {
            let load_option: u16 = format.parse_boot_next(&load_option)?;

            let load_option = adapter.get_load_option(load_option)?;

            match load_option {
                None => println!("unknown"),
                Some(load_option) => {
                    println!("{:04X} {}", load_option.number, load_option.description)
                }
            }

            Ok(())
        }
        Command::Get => {
            let boot_next = adapter.get_boot_next()?;
            match boot_next {
                None => println!("unset"),
                Some(boot_next) => println!("{:04X}", boot_next),
            }
            Ok(())
        }
        Command::Set {
            format,
            load_option,
        } => {
            let load_option: u16 = format.parse_boot_next(&load_option)?;

            adapter.set_boot_next(load_option)?;

            println!("{:04X}", load_option);
            Ok(())
        }
    }
}

/// The format of the `BootNext` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum LoadOptionFormat {
    /// The value is a hex number.
    #[value(alias("h"))]
    #[value(alias("hexadecimal"))]
    Hex,
    /// The value is a decimal number.
    #[value(alias("d"))]
    #[value(alias("decimal"))]
    Dec,
}

impl LoadOptionFormat {
    /// Radix of the underlying numeric format.
    pub fn radix(&self) -> u32 {
        match self {
            LoadOptionFormat::Hex => 16,
            LoadOptionFormat::Dec => 10,
        }
    }

    /// Parse the `BootNext` value using the format.
    pub fn parse_boot_next(&self, value: &str) -> Result<u16, anyhow::Error> {
        let val = u16::from_str_radix(value, self.radix()).map_err(|err| {
            anyhow::format_err!("unable to parse the boot option in {self} format: {err}")
        })?;
        Ok(val)
    }
}

impl std::fmt::Display for LoadOptionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dec => f.write_str("decimal"),
            Self::Hex => f.write_str("hex"),
        }
    }
}
