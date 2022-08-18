// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);

    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_italic();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Hello", &format1)?;
    worksheet.write_string(1, 0, "World", &format2)?;

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Hello", &format2)?;
    worksheet.write_string(1, 0, "World", &format1)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn bootstrap18_duplicate_formats_in_different_order() {
    let testcase = "bootstrap18";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    _ = create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}