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

// Test to demonstrate set_selection() and merged cells.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_align(FormatAlign::Center);

    worksheet.set_selection(3, 0, 3, 0)?;

    worksheet.merge_range(0, 0, 1, 0, "col1", &format)?;
    worksheet.merge_range(0, 1, 1, 1, "col2", &format)?;
    worksheet.merge_range(0, 2, 1, 2, "col3", &format)?;
    worksheet.merge_range(0, 3, 1, 3, "col4", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_merge_cells01() {
    let test_runner = common::TestRunner::new()
        .set_name("merge_cells01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
