use chrono::{Days, NaiveDate};
use log::info;
use read_xlsx::read_util::read_xlsx;
use read_xlsx::write_util::write_csv;

fn main() {

    // 日志打印
    tracing_subscriber::fmt().init();

    let file_path = "./mulu.xlsx";
    // 读取数据
    let (start_time, data, end_time) = read_xlsx(file_path);

    // 开始日期
    let mut start = NaiveDate::parse_from_str(&start_time, "%Y.%m.%d").unwrap();
    // 结束日期
    let end = NaiveDate::parse_from_str(&end_time, "%Y.%m.%d").unwrap();

    // 收集不存在的日期
    let mut nonexistent = Vec::<String>::new();
    // 判断是否小于结束时间
    while start.lt(&end) {
        let start_str = start.format("%Y.%m.%d").to_string();
        if !data.contains(&start_str) {
            nonexistent.push(start_str.clone());
            info!("缺少日志：{}",start_str);
        }
        // 加一天
        start = start.checked_add_days(Days::new(1)).unwrap();
    }

    let data: Vec<String> = data.into_iter().collect();

    let error_data = "./error_date.csv";
    let ok_data = "./ok_date.csv";

    //读取xlsx数据
    info!("写入缺少日期到{}文件中",error_data);
    write_csv(error_data, nonexistent);

    //读取xlsx数据
    info!("写入存在日期到{}文件中",file_path);
    write_csv(ok_data, data);

    info!("数据处理已完成");
}
