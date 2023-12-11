
//! common SAP processes

pub mod matlcons;
mod test;

pub use test::Test;

/// Operation standard functions
pub trait CliOperation {
    /// run the operation as configured
    fn exec() -> Result<(), Box<dyn std::error::Error>>;

    /// train the operation's bot
    fn train() -> Result<(), Box<dyn std::error::Error>>;
}
