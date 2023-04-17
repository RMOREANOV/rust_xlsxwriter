// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxColor, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test case to demonstrate creating a basic file with theme colors.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for row in 0..=5u32 {
        let col = 0u16;
        let color = (col + 2) as u8;
        let shade = row as u8;
        let theme_color = XlsxColor::Theme(color, shade);
        let color_format = Format::new().set_background_color(theme_color);

        worksheet.write_blank(row, col, &color_format)?;
    }

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_theme_color03() {
    let test_runner = common::TestRunner::new()
        .set_name("theme_color03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
