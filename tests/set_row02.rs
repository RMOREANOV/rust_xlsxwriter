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

    worksheet.set_row_height_pixels(0, 1)?;
    worksheet.set_row_height_pixels(1, 2)?;
    worksheet.set_row_height_pixels(2, 3)?;
    worksheet.set_row_height_pixels(3, 4)?;

    worksheet.set_row_height_pixels(11, 12)?;
    worksheet.set_row_height_pixels(12, 13)?;
    worksheet.set_row_height_pixels(13, 14)?;
    worksheet.set_row_height_pixels(14, 15)?;

    worksheet.set_row_height_pixels(18, 19)?;
    worksheet.set_row_height_pixels(20, 21)?;
    worksheet.set_row_height_pixels(21, 22)?;

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
