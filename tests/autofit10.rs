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

// Test to demonstrate autofit.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let default = Format::default();
    let bold = Format::new().set_bold();

    let segments = [
        (&default, "F"),
        (&bold, "o"),
        (&default, "o"),
        (&bold, "b"),
        (&default, "a"),
        (&bold, "r"),
    ];
    worksheet.write_rich_string(0, 0, &segments)?;

    worksheet.write_string_with_format(1, 0, "Bar", &bold)?;

    worksheet.autofit();

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofit10() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit10")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
