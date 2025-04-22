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
use flate2::write::GzEncoder;
use flate2::Compression;
use needletail::{parse_fastx_file, parse_fastx_reader};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};

#[derive(Debug, Args)]
pub struct FilterArgs {
    ///vcf file
    pub fastx_file: Option<String>,

    ///output file
    #[arg(short, long, value_name = "OUT_FASTQ_FILE")]
    out_fastq_file: Option<String>,

    ///filter by minimum quality score
    #[arg(
        short = 'q',
        long,
        default_value_t = 0,
        value_name = "Minimum read Q-score"
    )]
    min_read_qscore: i32,

    ///filter by minimum quality score
    #[arg(
        short = 'l',
        long,
        default_value_t = 0,
        value_name = "Minimum read length"
    )]
    min_read_length: i64,

    ///filter by maximum quality score
    #[arg(
        short = 'Q',
        long,
        default_value_t = std::i32::MAX,
        value_name = "Maximum read Q-score"
    )]
    max_read_qscore: i32,

    ///filter by maximum read length
    #[arg(
        short = 'L',
        long,
        default_value_t = std::i64::MAX,
        value_name = "Maximum read length"
    )]
    max_read_length: i64,
}

/// .
#[doc = r"Filter sequences"]
pub fn run(options: FilterArgs) {
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

    let mut gz_encoder;
    let mut fastq_writer: BufWriter<Box<dyn Write>> = match options.out_fastq_file {
        Some(filename) => {
            if filename.ends_with(".gz") {
                let file = File::create(filename).expect("Error: cannot create output file");
                gz_encoder = GzEncoder::new(file, Compression::default());
                BufWriter::new(Box::new(&mut gz_encoder))
            } else {
                let file = File::create(filename).expect("Error: cannot create output file");
                BufWriter::new(Box::new(file))
            }
        }
        None => BufWriter::new(Box::new(io::stdout())),
    };

    while let Some(record) = fastx_reader.next() {
        let seqrec = record.expect("invalid record");

        //compute average quality score
        let mut sum: f64 = 0.0;
        let len: i64 = seqrec.num_bases() as i64;

        let qscores = seqrec.qual().expect("Error getting quality scores");
        let base: f64 = 10.0;
        for q in qscores {
            sum += base.powf((-((*q - 33) as f64)) / 10.0);
        }
        let avg = sum / len as f64;
        let readq: f64 = -10.0 * avg.log10();

        let mut keep = true;

        //to write a feature natural filter expression mechanism
        if readq < options.min_read_qscore as f64 {
            keep = false;
        }
        if len < options.min_read_length {
            keep = false;
        }
        if readq > options.max_read_qscore as f64 {
            keep = false;
        }
        if len > options.max_read_length {
            keep = false;
        }

        if keep {
            let id: String = match std::str::from_utf8(seqrec.id()) {
                Ok(string_slice) => string_slice.to_string(),
                Err(_error) => String::new(),
            };
            let seq = seqrec.seq();
            let qual = seqrec.qual().expect("Error: cannot get quality scores");

            writeln!(fastq_writer, "@{}", id).expect("Cannot write out ID");
            writeln!(
                fastq_writer,
                "{}",
                seq.iter().map(|&c| c as char).collect::<String>()
            )
            .expect("Cannot write out sequences");
            writeln!(fastq_writer, "+").expect("Cannot write out +");
            writeln!(
                fastq_writer,
                "{}",
                qual.iter().map(|&q| (q + 0) as char).collect::<String>()
            )
            .expect("Cannot write out qualities");
        }
    }
}
