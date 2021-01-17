/**
 * author(s):   @urbanij
 * date:        Sun 17 Jan 2021 23:06:12 CET
 *
 * description: Generate a messages-density histogram of your exported Telegram chat.
 *              └── why? I was curious, and Python was too slow(*) at parsing timestamps and so on.
 *
 *                    (*) 0.23 seconds vs 45 seconds benchmarked on the same ~45k messages dataset.
 */

 
use std::fs; // renaming
use std::fs::File;
use std::io::prelude::*;
// use std::fs::OpenOptions; // opening file

use chrono::NaiveDateTime;
// use chrono::{DateTime, NaiveDate, NaiveTime};
use chrono::format::ParseError;

use regex::Regex;

use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

const NUM_BINS: usize = 200;


fn main() {
        
    let REGEX_DATE_PATTERN = Regex::new(r"(\d{2}.\d{2}.\d{4} \d{2}:\d{2}:\d{2})").unwrap();
    
    let mut timestamps = vec![];

    rename_file("messages.html", "messages1.html").unwrap();

    let mut i = 1;
    while let Ok(content) = read_file( format!("messages{}.html", i).as_str() ) {
        println!("Processing {}", format!("messages{}.html", i));
        i += 1;

        for caps in REGEX_DATE_PATTERN.captures_iter(content.as_str()) {
            
            let epoch_date = tg_date_to_epoch_date(caps.get(1).unwrap().as_str());
            match epoch_date {
                Ok(n) => {
                    // println!("{:?}, ", n);
                    timestamps.push(n as f64);
                },
                Err(e) => println!("Error: {}", e),
            }

        }
    }

    generate_plot(&timestamps);

    rename_file("messages1.html", "messages.html").unwrap();
}

fn generate_plot(data: &Vec<f64>) {
    let h = Histogram::from_slice(&data, HistogramBins::Count(NUM_BINS))
        .style(&BoxStyle::new().fill("burlywood"));

    let v = ContinuousView::new().add(h);

    Page::single(&v).save("histogram.svg").expect("saving svg");
}

fn rename_file(original: &str, new_name: &str) -> std::io::Result<()> {
    fs::rename(original, new_name)?;
    println!("Renaming {} to {}", original, new_name);
    Ok(())
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    // println!("{}", content);
    Ok(content)
}

fn tg_date_to_epoch_date(tg_date: &str) -> Result<i64, ParseError> {
    let ts = NaiveDateTime::parse_from_str(tg_date, "%d.%m.%Y %H:%M:%S")?;
    Ok(ts.timestamp())
}

