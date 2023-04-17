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

// Test to demonstrate simple hyperlinks needing escapes.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::default();

    // Blank space in the url.
    worksheet.write_url_with_options(0, 0, "http://google.com/some link", "", "", Some(&format))?;

    workbook.save(filename)?;

    Ok(())
}

// Test to demonstrate simple hyperlinks, with escapes
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::default();

    // Blank space in the url.
    worksheet.write_url_with_options(
        0,
        0,
        "http://google.com/some%20link",
        "http://google.com/some link",
        "",
        Some(&format),
    )?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_hyperlink17_1() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink17")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_hyperlink17_2() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink17")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
