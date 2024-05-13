#[macro_use]
extern crate prettytable;

use clap::Parser;
use invoicing_lib::types::{invoice::Invoice, invoice_line_item::InvoiceLineItem};

use crate::{
    fomatters::{format_invoice_line_items_table, format_invoices_table},
    storage::{read_storage_from_file, write_storage_to_file},
    types::{Args, Commands, Storage},
};

mod constants;
mod fomatters;
mod storage;
mod types;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let default_storage = Storage {
        invoices: vec![
            Invoice {
                number: "I0001".to_owned(),
                date: chrono::Utc::now().date_naive(),
                name: "Some Company".to_owned(),
                zip_code: "1234".to_owned(),
                city: "New York".to_owned(),
                street: "Longstreet".to_owned(),
                country: None,
                items: vec![InvoiceLineItem {
                    name: "Travel Cost".to_owned(),
                    amount: 1,
                    net_value: 321.63,
                    vat_percentage: 19.0,
                }],
            },
            Invoice {
                number: "I0002".to_owned(),
                date: chrono::Utc::now().date_naive(),
                name: "Some Company".to_owned(),
                zip_code: "1234".to_owned(),
                city: "New York".to_owned(),
                street: "Longstreet".to_owned(),
                country: None,
                items: vec![
                    InvoiceLineItem {
                        name: "Cake".to_owned(),
                        amount: 2,
                        net_value: 12.22,
                        vat_percentage: 19.0,
                    },
                    InvoiceLineItem {
                        name: "Drinks".to_owned(),
                        amount: 5,
                        net_value: 2.52,
                        vat_percentage: 19.0,
                    },
                ],
            },
        ],
    };

    let storage = match read_storage_from_file() {
        Ok(storage) => storage,
        Err(e) => {
            eprintln!("Could not read storage from file {e}");
            println!("Generating new storage file with default valiues.");
            default_storage
        }
    };

    match args.command {
        Commands::List => {
            let table = format_invoices_table(&storage.invoices);
            println!("{table}");
        }
        Commands::Total => {
            let total: f64 = storage
                .invoices
                .iter()
                .map(|i: &Invoice| i.total_amount())
                .sum();

            let count: usize = storage.invoices.len();
            println!("{count} invoices with a total of €{total:.2}");
        }
        Commands::Detail { invoice_number } => {
            let invoice = storage.invoices.iter().find(|x| x.number == invoice_number);
            match invoice {
                Some(invoice) => {
                    let invoice = invoice.clone();
                    let table = format_invoices_table(&vec![invoice.clone()]);
                    println!("{table}");

                    let items_table = format_invoice_line_items_table(&invoice);
                    println!("Items: ");
                    println!("{items_table}");
                }
                None => println!("No invoice with number '{invoice_number}' in the system"),
            }
        }
    };

    write_storage_to_file(&storage)?;

    Ok(())
}
