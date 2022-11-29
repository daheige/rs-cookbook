use csv::Error;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Debug)]
struct Record {
    year: u16,
    make: String,
    model: String,
    desc: String,
}

#[derive(Serialize)]
struct LocationEntry<'a> {
    name: &'a str,
    place: &'a str,
    id: i64,
}

fn main() -> Result<(), Error> {
    let s = "year,make,model,description
    1948,Porsche,356,car
    1967,Ford,Mustang fastback 1967,car2";
    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for row in reader.records() {
        let record = row?;
        // println!("{:?}",record);
        println!("in {},{} built the {} model.it is a {}.",
                 &record[0].trim(),
                 &record[1],
                 &record[2],
                 &record[3],
        );
    }

    let s = "year,make,model,description
    1948,Porsche,356,car
    1967,Ford,Mustang fastback 1967,car2";
    // 解析数据到结构体
    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for row in reader.records() {
        let record = row?;
        let r = Record {
            year: (&record[0].trim()).parse().unwrap(),
            make: (&record[1]).to_string(),
            model: (&record[2]).to_string(),
            desc: (&record[3]).to_string(),
        };

        println!("r: {:?}", r);
    }

    // 将记录序列化为csv
    /*
    csv::writer 支持从 Rust 类型到 CSV 记录的自动序列化。write_record 只写入包含字符串数据的简单记录。
    具有更复杂值（如数字、浮点和选项）的数据使用 serialize 进行序列化。因为 csv::writer 使用内部缓冲区，
    所以在完成时总是显式刷新 flush
     */
    let mut wrt = csv::Writer::from_writer(io::stdout());
    wrt.write_record(&["name", "place", "id"]);
    wrt.serialize(("mark", "sydney", 87))?;
    wrt.serialize(("alex", "dublin", 132))?;
    wrt.serialize(("ak", "delhi", 11))?;

    // 使用 serde crate 将自定义结构体序列化为 CSV 记录
    let rec1 = LocationEntry{
        name:"mark2",place:"mel",id:121,
    };
    wrt.serialize(rec1)?;
    wrt.flush()?;

    Ok(())
}
