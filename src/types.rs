use clap::{Parser, Subcommand};
use invoicing_lib::types::invoice::Invoice;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Commands {
    /// List all invoices in the system
    List,
    /// List invoice number with total amount
    Total,
    /// List detail information of a single invoice
    Detail { invoice_number: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub invoices: Vec<Invoice>,
}
