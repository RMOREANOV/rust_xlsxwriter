// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate various print options.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_print_center_horizontally(true);

    worksheet.write_string(0, 0, "Foo")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_print_options02() {
    let test_runner = common::TestRunner::new()
        .set_name("print_options02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
