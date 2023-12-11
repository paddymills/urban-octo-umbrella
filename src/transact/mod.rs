
//! SAP transactions

use std::fmt::Display;

/// SAP transactions
#[derive(Debug)]
pub enum TransAct {
    /// CO02: edit order
    CO02,
    /// CO13: cancel production order confirmation
    CO13,
    /// MD51: run MRP
    MD51,
}

impl Display for TransAct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TransAct::CO02 => write!(f, "CO02"),
            TransAct::CO13 => write!(f, "CO13"),
            TransAct::MD51 => write!(f, "MD51"),
        }
    }
}

pub mod co02;
