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

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let centered = Format::new()
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter);

    worksheet.write_string_with_format(1, 1, "Foo", &centered)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format11() {
    let test_runner = common::TestRunner::new()
        .set_name("format11")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
