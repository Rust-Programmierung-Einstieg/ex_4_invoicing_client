use invoicing_lib::types::invoice::Invoice;
use prettytable::{Cell, Row, Table};
pub fn format_invoices_table(invoices: &Vec<Invoice>) -> String {
    let mut table = Table::new();
    table.add_row(row!["Number", "Date", "Name", "Vat / €", "Total / €"]);

    for i in invoices {
        table.add_row(row![
            i.number,
            i.date,
            i.name,
            format!("{:.2}", i.vat_amount()),
            format!("{:.2}", i.total_amount()),
        ]);
    }

    table.to_string()
}
pub fn format_invoice_line_items_table(invoice: &Invoice) -> String {
    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Amount",
        "Net / €",
        "Vat / %",
        "Vat / €",
        "Total / €"
    ]);

    for i in &invoice.items {
        table.add_row(row![
            i.name,
            i.amount,
            i.net_value,
            i.vat_percentage,
            format!("{:.2}", i.vat_amount()),
            format!("{:.2}", i.total_amount()),
        ]);
    }

    table.to_string()
}
