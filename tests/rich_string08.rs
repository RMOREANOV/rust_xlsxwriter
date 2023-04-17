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

// Test to demonstrate rich strings.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let default = Format::default();
    let center = Format::new().set_align(FormatAlign::Center);

    worksheet.write_string_with_format(0, 0, "Foo", &bold)?;
    worksheet.write_string_with_format(1, 0, "Bar", &italic)?;

    let segments = [(&default, "ab"), (&bold, "cd"), (&default, "efg")];
    worksheet.write_rich_string_with_format(2, 0, &segments, &center)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_rich_string08() {
    let test_runner = common::TestRunner::new()
        .set_name("rich_string08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
