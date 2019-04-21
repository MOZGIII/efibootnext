#[derive(Debug, Fail)]
#[fail(display = "load option with number {:04X} does not exist", number)]
pub struct NoSuchLoadOption {
    pub number: u16,
}

impl NoSuchLoadOption {
    pub fn new(number: u16) -> Self {
        Self { number: number }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "boot next value is not valid")]
pub struct InvalidBootNextValue;
