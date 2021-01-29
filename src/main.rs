/**
 * author(s):   @urbanij
 * date:        Sun 17 Jan 2021 23:06:12 CET
 *
 */

use argparse::{ArgumentParser, StoreTrue, Store};

mod utils;

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

    match utils::read_file("messages.html") {
        Ok(content) => utils::process_content(content, &mut timestamps, None, verbose),
        Err(e) => {
            println!("{:?}", e);
            println!("[-] `messages.html` not found!");
            println!("    Make sure you're running `telegrust-histo` inside the root directory of your exported chat.");
            std::process::exit(exitcode::DATAERR);
        },
    }
    
    let mut num_files = 2;
    while let Ok(content) = utils::read_file( format!("messages{}.html", num_files).as_str() ) {
        utils::process_content(content, &mut timestamps, Some(num_files), verbose);
        num_files += 1;
    }

    // println!("{:?}", timestamps);
    if timestamps.len() <= 0 {
        eprintln!("[-] Some errors with your files... Quitting.");
        std::process::exit(exitcode::DATAERR);
    }

    utils::generate_plot(&timestamps, num_bins);
    
    println!("[+] Processed {} files ({} messages) from {} to {}", 
        num_files - 1,
        timestamps.len(),
        utils::epoch_to_readable_date(timestamps[0]),
        utils::epoch_to_readable_date(timestamps[timestamps.len()-1]),
    );

}
