// use std::fs; // renaming
use std::fs::File;
use std::io::prelude::*;
// use std::fs::OpenOptions; // opening file

use chrono::NaiveDateTime;
use chrono::format::ParseError;

use regex::Regex;

use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};



pub fn generate_plot(data: &Vec<f64>, num_bins: usize) 
{
    let h = Histogram::from_slice(
                            &data, 
                            HistogramBins::Count(num_bins))
                                .style(&BoxStyle::new().fill("burlywood"));
    
    // let data2: Vec<f64> = data.iter().map(|&i| i*0.9).collect();
    // let h2 = Histogram::from_slice(
    //                             &data2, 
    //                             HistogramBins::Count(num_bins))
    //                                 .style(&BoxStyle::new());

    let v = ContinuousView::new()
        .add(h)
        .x_label("date (epoch time)")
        .y_label("count");

    let file_name = format!("histogram_{}.svg", num_bins);

    Page::single(&v).save(&file_name).expect("saving svg");

    println!("[+] Generated {} (using {} bins) in the current folder.", file_name, num_bins);
}

// pub fn rename_file(original: &str, new_name: &str, verbose: bool) -> std::io::Result<()> {
//     fs::rename(original, new_name)?;
//     if verbose {
//         println!("Renamed {} to {}", original, new_name);
//     }
//     Ok(())
// }

pub fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn tg_date_to_epoch_date(tg_date: &str) -> Result<i64, ParseError> {
    let ts = NaiveDateTime::parse_from_str(tg_date, "%d.%m.%Y %H:%M:%S")?;
    Ok(ts.timestamp())
}

pub fn epoch_to_readable_date(epoch_date: f64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(epoch_date as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M").to_string();

    timestamp_str
}

pub fn process_content(content: String, timestamps: &mut Vec<f64>, file_num: Option<i32>, verbose: bool) {
    
    let regex_date_pattern = Regex::new(r"(\d{2}.\d{2}.\d{4} \d{2}:\d{2}:\d{2})").unwrap();
                                    // matches timestamps of this kind: 13.12.2018 19:17:39
    
    if verbose {
        match file_num {
            Some(v) => println!("Processing messages{}.html", v),
            None => println!("Processing messages.html"),
        };
    }

    for caps in regex_date_pattern.captures_iter(content.as_str()) {
        let epoch_date = tg_date_to_epoch_date(caps.get(1).unwrap().as_str());
        match epoch_date {
            Ok(n) => timestamps.push(n as f64),
            Err(e) => println!("Error: {}", e),
        }
    }
}
