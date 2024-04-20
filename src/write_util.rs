// 出入csv文件
pub fn write_csv(path:&str, data: Vec<String>) {

    let mut writer = csv::Writer::from_path(path).unwrap();

    for (index, value) in data.iter().enumerate() {
        writer.write_record(&[format!("{}", index), value.clone()]).expect("写入失败");
    }
    writer.flush().unwrap();
}