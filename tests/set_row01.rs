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

// Test to demonstrate row formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(0, 0.75)?;
    worksheet.set_row_height(1, 1.50)?;
    worksheet.set_row_height(2, 2.25)?;
    worksheet.set_row_height(3, 3)?;

    worksheet.set_row_height(11, 9)?;
    worksheet.set_row_height(12, 9.75)?;
    worksheet.set_row_height(13, 10.50)?;
    worksheet.set_row_height(14, 11.25)?;

    worksheet.set_row_height(18, 14.25)?;
    worksheet.set_row_height(20, 15.75)?;
    worksheet.set_row_height(21, 16.50)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_set_row01() {
    let test_runner = common::TestRunner::new()
        .set_name("set_row01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
