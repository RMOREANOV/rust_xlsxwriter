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

    let worksheet = workbook.add_worksheet();

    let image1 = Image::new("tests/input/images/red.jpg")?;
    let image2 = Image::new("tests/input/images/blue.jpg")?;
    let image3 = Image::new("tests/input/images/yellow.jpg")?;

    worksheet.set_footer("&L&G&C&G&R&G");
    worksheet.set_footer_image(&image1, HeaderImagePosition::Left)?;
    worksheet.set_footer_image(&image2, HeaderImagePosition::Center)?;
    worksheet.set_footer_image(&image3, HeaderImagePosition::Right)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image04() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
