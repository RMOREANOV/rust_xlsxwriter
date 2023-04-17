// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{HeaderImagePosition, Image, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate adding header/footer images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet1 = workbook.add_worksheet();
    worksheet1.set_portrait();

    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_portrait();

    let worksheet3 = workbook.add_worksheet();
    worksheet3.set_portrait();

    let image = Image::new("tests/input/images/red.jpg")?;

    worksheet3.set_header("&L&G");
    worksheet3.set_header_image(&image, HeaderImagePosition::Left)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image21() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image21")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
