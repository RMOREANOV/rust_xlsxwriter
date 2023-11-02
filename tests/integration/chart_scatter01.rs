// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartAxisCrossing, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Scatter);
    chart.set_axis_ids(40262272, 40260352);
    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 1, 4, 1));

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 2, 4, 2));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Scatter);
    chart.set_axis_ids(40262272, 40260352);
    chart
        .add_series()
        .set_categories("=Sheet1!$A$1:$A$5")
        .set_values("=Sheet1!$B$1:$B$5");

    chart
        .add_series()
        .set_categories("=Sheet1!$A$1:$A$5")
        .set_values("=Sheet1!$C$1:$C$5");

    worksheet.insert_chart(8, 4, &chart)?;

    // These crossing types are on the wrong axes and should be ignored.
    chart
        .x_axis()
        .set_crossing(ChartAxisCrossing::CategoryNumber(3));
    chart
        .y_axis()
        .set_crossing(ChartAxisCrossing::CategoryNumber(8));

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_scatter01_1() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_scatter01")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_chart_scatter01_2() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_scatter01")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
