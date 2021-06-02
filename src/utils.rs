use std::io::prelude::*;
// use std::fs::OpenOptions; // opening file
use chrono::NaiveDateTime;
use chrono::format::ParseError;

use regex::Regex;

use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

use crate::message::Message;

pub fn generate_plot(messages: &[Message], num_bins: usize) {
    
    // accumulate timestamps into `data`.
    let data = messages.iter().map(|i| i.timestamp).collect::<Vec<f64>>();

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
    let mut file = std::fs::File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn tg_date_to_epoch_date(tg_date: &str) -> Result<i64, ParseError> {
    let ts = NaiveDateTime::parse_from_str(tg_date, "%d.%m.%Y %H:%M:%S")?;
    Ok(ts.timestamp())
}

pub fn process_content(
    content: String,
    messages: &mut Vec<Message>, 
    file_num: Option<i32>, 
    verbose: bool) 
{
    
    // matches messages of this kind: 13.12.2018 19:17:39
    let regex_date_pattern = Regex::new(r"(\d{2}.\d{2}.\d{4} \d{2}:\d{2}:\d{2})")
                                            .unwrap(); // unwrap is risky but this should never fail.
    
    if verbose {
        match file_num {
            Some(v) => println!("Processing messages{}.html", v),
            None => println!("Processing messages.html"),
        };
    }


    // TODO: get todays epoch date anc compare it with the date found in the text message: based on that add it or not.
    
    for caps in regex_date_pattern.captures_iter(content.as_str()) {
        let epoch_date = tg_date_to_epoch_date(caps.get(1).unwrap().as_str());
        match epoch_date {
            Ok(n) => {
                if n != 0 {

                }
                messages.push(Message::new(n as f64))
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
