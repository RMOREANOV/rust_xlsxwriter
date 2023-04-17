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

// Test case to demonstrate creating a basic file with 3 worksheets and no data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let _worksheet = workbook.add_worksheet().set_name("Foo");
    let _worksheet = workbook.add_worksheet();
    let _worksheet = workbook.add_worksheet().set_name("Bar");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap03_multiple_worksheets_with_names() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
