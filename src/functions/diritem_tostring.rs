use std::path::Path;

use chrono::{Datelike, Local, TimeZone, Timelike};
use goodmorning_bindings::services::v1::{ItemVisibility, V1DirItem};

pub fn diritem_tostring(item: &V1DirItem, max_size_len: usize, path: &Path) -> String {
    let file = format!("{: <4}", if item.is_file { "file" } else { "dir" });
    let inherited = format!("{: <1}", if item.visibility.inherited { "" } else { "*" });
    let visibility = format!(
        "{: <8}",
        match item.visibility.visibility {
            ItemVisibility::Hidden => "hidden",
            ItemVisibility::Public => "public",
            ItemVisibility::Private => "private",
        }
    );
    let size = item.size.to_string();
    let size_pad = " ".repeat(max_size_len - size.len());

    let localtime = Local.timestamp_opt(item.last_modified as i64, 0).unwrap();
    let min = format!("{:0>2}", localtime.minute());
    let hour = format!("{:0>2}", localtime.hour());
    let day = format!("{: <2}", localtime.day());
    let month = format!("{: <4}", month_abbrev(localtime.month() as u8));
    let year = localtime
        .year()
        .to_string()
        .as_bytes()
        .iter()
        .rev()
        .take(2)
        .rev()
        .map(|b| *b as char)
        .collect::<String>();

    format!(
        "{file} {visibility}{inherited} {size_pad}{size} {year} {month} {day} {hour}:{min} {}",
        path.join(&item.name).to_str().unwrap(),
    )
}

fn month_abbrev(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sept",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    }
}
