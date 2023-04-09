// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, FormatBorder, FormatDiagonalBorder, Workbook, XlsxColor, XlsxError};

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format1 = Format::new()
        .set_font_color(XlsxColor::Automatic)
        .set_border_diagonal(FormatBorder::Thin)
        .set_border_diagonal_type(FormatDiagonalBorder::BorderDown)
        .set_border_diagonal_color(XlsxColor::Automatic);

    worksheet.write_with_format(0, 0, "Foo", &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format23() {
    let test_runner = common::TestRunner::new()
        .set_name("format23")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
