use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Locale {
    US,
    Global,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}

#[derive(Debug)]
pub enum Market {
    Stocks,
    Crypto,
    FX,
    OTC,
    Indices,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self).to_lowercase())
    }
}
