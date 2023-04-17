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

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::default();

    worksheet.write_url_with_options(
        0,
        0,
        r"file:///\\VBOXSVR\share\foo.xlsx",
        r"J:\foo.xlsx",
        "",
        Some(&format),
    )?;
    worksheet.write_url_with_options(2, 0, r"file:///foo.xlsx", "", "", Some(&format))?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_hyperlink07() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
