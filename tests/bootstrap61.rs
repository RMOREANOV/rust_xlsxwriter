// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use chrono::NaiveDate;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test case generic write_with_format().
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_num_format("yyyy\\-mm\\-dd\\ hh:mm:ss");
    let format2 = Format::new().set_num_format("yyyy\\-mm\\-dd;@");
    let format3 = Format::new().set_num_format("hh:mm:ss;@");

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30)?;

    let datetime = NaiveDate::from_ymd_opt(2023, 1, 25)
        .unwrap()
        .and_hms_opt(18, 0, 0)
        .unwrap();
    let time = datetime.time();

    let datetime2 = NaiveDate::from_ymd_opt(2023, 1, 25)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let date = datetime2.date();

    worksheet.write_with_format(0, 0, &datetime, &format1)?;
    worksheet.write_with_format(1, 0, &date, &format2)?;
    worksheet.write_with_format(2, 0, &time, &format3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case generic write().
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30)?;

    let datetime = NaiveDate::from_ymd_opt(2023, 1, 25)
        .unwrap()
        .and_hms_opt(18, 0, 0)
        .unwrap();
    let time = datetime.time();

    let datetime2 = NaiveDate::from_ymd_opt(2023, 1, 25)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let date = datetime2.date();

    worksheet.write(0, 0, &datetime)?;
    worksheet.write(1, 0, &date)?;
    worksheet.write(2, 0, &time)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap61_date_time_1() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap61")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap61_date_time_2() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap61")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
