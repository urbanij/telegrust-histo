/**
 * author(s):   @urbanij
 * date:        Sun 17 Jan 2021 23:06:12 CET
 *
 */

 
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

use argparse::{ArgumentParser, StoreTrue, Store};

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};


fn generate_plot(
    data: &Vec<f64>, 
    num_bins: usize) 
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

// fn rename_file(original: &str, new_name: &str, verbose: bool) -> std::io::Result<()> {
//     fs::rename(original, new_name)?;
//     if verbose {
//         println!("Renamed {} to {}", original, new_name);
//     }
//     Ok(())
// }

fn read_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn tg_date_to_epoch_date(tg_date: &str) -> Result<i64, ParseError> {
    let ts = NaiveDateTime::parse_from_str(tg_date, "%d.%m.%Y %H:%M:%S")?;
    Ok(ts.timestamp())
}

fn epoch_to_readable_date(epoch_date: f64) -> String {
    // Creates a new SystemTime from the specified number of whole seconds
    let d = UNIX_EPOCH + Duration::from_secs(epoch_date as u64);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M").to_string();

    timestamp_str
}

fn process_content(content: String, timestamps: &mut Vec<f64>, file_num: Option<i32>, verbose: bool) {
    
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

fn main() {

    // default values of argparsed arguments
    let mut verbose = false;
    let mut num_bins = 200;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("[telegrust-histo](https://github.com/urbanij/telegrust-histo)");
        ap.refer(&mut num_bins)
            .add_option(&["-b"], 
                        Store,
                        "num_bins in your histogram");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], 
                        StoreTrue,
                        "Be verbose");
        ap.parse_args_or_exit();
    }
        
    
    
    let mut timestamps = vec![];

    // first file has no number in it, the following have an incremental suffix 
    // before the file extension, thus i iterate those in a loop.

    match read_file("messages.html") {
        Ok(content) => process_content(content, &mut timestamps, None, verbose),
        Err(e) => {
            println!("{:?}", e);
            println!("[-] `messages.html` not found!");
            println!("    Make sure you're running `telegrust-histo` inside the root directory of your exported chat.");
            std::process::exit(exitcode::DATAERR);
        },
    }
    
    let mut num_files = 2;
    while let Ok(content) = read_file( format!("messages{}.html", num_files).as_str() ) {
        process_content(content, &mut timestamps, Some(num_files), verbose);
        num_files += 1;
    }

    // println!("{:?}", timestamps);
    if timestamps.len() <= 0 {
        eprintln!("[-] Some errors with your files... Quitting.");
        std::process::exit(exitcode::DATAERR);
    }

    generate_plot(&timestamps, num_bins);
    
    println!("[+] Processed {} files ({} messages) from {} to {}", 
        num_files - 1,
        timestamps.len(),
        epoch_to_readable_date(timestamps[0]),
        epoch_to_readable_date(timestamps[timestamps.len()-1]),
    );

}
