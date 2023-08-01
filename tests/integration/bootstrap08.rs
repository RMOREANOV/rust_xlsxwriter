// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet1 = workbook.add_worksheet();

    worksheet1.write_string(0, 0, "Hello")?;
    worksheet1.write_string(1, 0, "World")?;
    worksheet1.write_string(2, 0, "Hello")?;
    worksheet1.write_string(3, 0, "World")?;

    let worksheet2 = workbook.add_worksheet();

    worksheet2.write_string(0, 0, "World")?;
    worksheet2.write_string(1, 0, "Hello")?;
    worksheet2.write_string(2, 0, "World")?;
    worksheet2.write_string(3, 0, "Hello")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap08_write_repeated_strings_on_different_worksheets() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}