// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Image, Workbook, XlsxError};

mod common;

// Test to demonstrate adding images to worksheets.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(1, 75)?;
    worksheet.set_column_width(2, 32)?;

    let mut image = Image::new("tests/input/images/logo.png")?;
    image.set_alt_text("logo.png");

    worksheet.insert_image_with_offset(1, 2, &image, 8, 5)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_image13() {
    let test_runner = common::TestRunner::new()
        .set_name("image13")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}