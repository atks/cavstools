// The MIT License
// Copyright (c) 2023 Adrian Tan <adrian_tan@nparks.gov.sg>
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the 'Software'), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

pub(crate) use clap::Args;
extern crate needletail;
use needletail::{parse_fastx_file, parse_fastx_reader};
use std::io;
use std::io::Write;

#[derive(Debug, Args)]
pub struct StatsArgs {
    ///vcf file
    pub fastx_file: Option<String>,
}

#[doc = r"Compute statistics for each read"]
pub fn run(options: StatsArgs) {
    let mut fastx_reader;
    match options.fastx_file {
        Some(filename) => {
            fastx_reader = parse_fastx_file(filename).expect("valid path/file");
        }
        None => {
            let stdin = io::stdin();
            fastx_reader = parse_fastx_reader(stdin).expect("Error: cannot read stdin");
        }
    }

    while let Some(record) = fastx_reader.next() {
        let seqrec = record.expect("invalid record");

        //compute average quality score
        let mut sum: f64 = 0.0;
        let mut count: i32 = 0;

        let qscores = seqrec.qual().expect("Error getting quality scores");
        let base: f64 = 10.0;
        for q in qscores {
            sum += base.powf((-((*q - 33) as f64)) / 10.0);
            count += 1;
        }
        let avg = sum / count as f64;
        let readq: f64 = -10.0 * avg.log10();

        let mut id_string = String::new();
        match std::str::from_utf8(seqrec.id()) {
            Ok(string_slice) => {
                id_string = string_slice.to_string();
                let words: Vec<&str> = id_string.split_whitespace().collect();
                id_string = words[0].to_string();
            }
            Err(_error) => {
                //do nothing
            }
        }

        writeln!(
            std::io::stdout(),
            "{}\t{}\t{}",
            id_string,
            readq.round(),
            count
        )
        .unwrap();
    }
}
