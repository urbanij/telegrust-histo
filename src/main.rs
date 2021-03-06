/**
 * author(s):   @urbanij
 * date:        Sun 17 Jan 2021 23:06:12 CET
 *
 */

use argparse::{ArgumentParser, StoreTrue, Store};

mod message;
mod utils;

fn main() {

    // default values of argparsed arguments
    let mut verbose = false;
    let mut num_bins = 200;
    let mut timespan = 0;

    {   // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("https://github.com/urbanij/telegrust-histo");
        ap.refer(&mut num_bins)
            .add_option(&["-b"], 
                        Store,
                        "num_bins in your histogram [default 200]");
        ap.refer(&mut timespan)
            .add_option(&["-t"], 
                        Store,
                        "messages timespan [default all]");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], 
                        StoreTrue,
                        "be verbose");
        ap.parse_args_or_exit();
    }
        
    
    
    let mut messages: Vec<message::Message> = vec![]; // unused as the name suggests, for now.

    // first file has no number in it, the following have an incremental suffix 
    // before the file extension, thus i iterate those in a loop.

    match utils::read_file("messages.html") {
        Ok(content) => utils::process_content(content.as_str(), &mut messages, None, verbose),
        Err(e) => {
            println!("{:?}", e);
            println!("[-] `messages.html` not found!");
            println!("    Make sure you're running `telegrust-histo` inside the root directory of your exported chat.");
            std::process::exit(exitcode::DATAERR);
        },
    }
    
    let mut num_files = 2;
    while let Ok(content) = utils::read_file( format!("messages{}.html", num_files).as_str() ) {
        utils::process_content(content.as_str(), &mut messages, Some(num_files), verbose);
        num_files += 1;
    }

    if messages.is_empty() {
        eprintln!("[-] Some errors with your file(s)... Quitting.");
        std::process::exit(exitcode::DATAERR);
    }

    utils::generate_plot(&messages, num_bins);
    
    println!("[+] Processed {} files ({} messages) from {} to {}", 
        num_files - 1,
        messages.len(),
        messages[0].get_timestamp_string(),
        messages[messages.len()-1].get_timestamp_string(),
    );

}
