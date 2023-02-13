// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{DocProperties, Workbook, XlsxError};

mod common;

// Test to demonstrate document properties.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let properties = DocProperties::new().set_custom_property("Checked by", "Adam");
    workbook.set_properties(&properties);

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 70)?;
    worksheet.write_string(
        0,
        0,
        r#"Select 'Office Button -> Prepare -> Properties' to see the file properties."#,
    )?;
    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_properties03() {
    let test_runner = common::TestRunner::new()
        .set_name("properties03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}