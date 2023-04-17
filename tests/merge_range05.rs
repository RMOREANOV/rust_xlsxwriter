// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate merged ranges.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::new().set_align(FormatAlign::Center);

    worksheet.merge_range(1, 1, 1, 3, "", &format)?;
    worksheet.write_number_with_format(1, 1, 123, &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_merge_range05() {
    let test_runner = common::TestRunner::new()
        .set_name("merge_range05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
