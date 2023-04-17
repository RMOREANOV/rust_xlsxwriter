// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "Hello")?;
    worksheet.write_string(1, 0, "World")?;
    worksheet.write_string(2, 0, "Hello")?;
    worksheet.write_string(3, 0, "World")?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate incremental saving and the get_worksheet*() methods.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let name = worksheet.name();

    // Test incremental save.
    worksheet.write_string(0, 0, "Hello")?;
    workbook.save(filename)?;

    let worksheet = workbook.worksheet_from_index(0)?;
    worksheet.write_string(1, 0, "World")?;
    workbook.save(filename)?;

    let worksheet = workbook.worksheet_from_index(0)?;
    worksheet.write_string(2, 0, "Hello")?;
    workbook.save(filename)?;

    let worksheet = workbook.worksheet_from_name(&name)?;
    worksheet.write_string(3, 0, "World")?;
    workbook.save(filename)?;

    Ok(())
}

// Write strings with generic write().
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "Hello")?;
    worksheet.write_string(1, 0, "World")?;
    worksheet.write_string(2, 0, "Hello")?;
    worksheet.write_string(3, 0, "World")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap07_write_repeated_strings() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap07")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap07_multi_save() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap07")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap07_with_generic_write() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap07")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
