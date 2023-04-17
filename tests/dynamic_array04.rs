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

// Test dynamic function with formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let bold = Format::new().set_bold();

    worksheet.write_dynamic_formula_with_format(0, 0, "=AVERAGE(TIMEVALUE(B1:B2))", &bold)?;
    worksheet.write_string(0, 1, "12:00")?;
    worksheet.write_string(1, 1, "12:00")?;
    worksheet.set_formula_result(0, 0, "0.5");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array04() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
