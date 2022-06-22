use calamine::DataType;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct InputRowRaw {
    #[serde(rename(deserialize = "other_name"))]
    #[serde(deserialize_with = "de_opt_native_datetime")]
    purchase_time: Option<NaiveDateTime>,
}

fn de_opt_native_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type_res = calamine::DataType::deserialize(deserializer);
    match data_type_res {
        Ok(data_type) => match data_type {
            DataType::DateTime(f) => Ok(DataType::DateTime(f).as_datetime()),
            _ => Ok(None),
        },
        Err(e) => Err(e),
    }
}
pub const HEADERS: &[&str] = &["other_name"];

#[cfg(test)]
mod tests {
    use calamine::{RangeDeserializerBuilder, Reader, Xlsx};

    use crate::{InputRowRaw, HEADERS};

    #[test]
    fn not_works() {
        let mut book: Xlsx<_> = calamine::open_workbook("workbook.xlsx").unwrap();
        let sheet = book.worksheet_range_at(0).unwrap().unwrap();
        let de = RangeDeserializerBuilder::with_headers(HEADERS)
            .from_range(&sheet)
            .unwrap();
        for row in de {
            let InputRowRaw { purchase_time } = row.unwrap();
            println!("{purchase_time:?}")
        }
    }
}
