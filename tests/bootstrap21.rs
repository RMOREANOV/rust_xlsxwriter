// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format = Format::new().set_font_strikethrough();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "Strikeout Text", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap21_strikethrough_text() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap21")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
