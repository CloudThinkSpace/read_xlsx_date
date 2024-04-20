use std::collections::BTreeSet;

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use log::info;

pub fn read_xlsx(path: &str) -> (String, BTreeSet<String>, String) {
    //读取xlsx数据
    info!("读取{}数据",path);
    // 打开文件
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();

    info!("数据读取中。。。。。。。。。。。。");
    // 读取sheet
    let range = workbook.worksheet_range("Sheet1").unwrap();

    let (rows, columns) = range.get_size();
    info!("数据表有：{}行，{}列", rows, columns);
    // 解析数据
    let iter_result = RangeDeserializerBuilder::new().from_range(&range).unwrap();
    // 定义结果
    let mut b_set = BTreeSet::<String>::new();
    // 开始时间
    let mut start_time = String::new();
    // 结束时间
    let mut end_time = String::new();

    for (_index, result) in iter_result.enumerate() {
        let (_colum1, colum2): (String, String) = result.unwrap();
        if b_set.is_empty() {
            start_time.push_str(&colum2);
        }
        end_time.clear();
        end_time.push_str(&colum2);
        if !b_set.contains(&colum2) {
            b_set.insert(colum2.clone());
        }
    }
    //读取xlsx数据
    info!("读取{}数据已完成",path);
    (start_time.to_string(), b_set, end_time.to_string())
}