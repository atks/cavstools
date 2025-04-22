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
use gb_io::reader::SeqReader;
use std::fs::File;
use std::io::{self, BufWriter, Write};

#[derive(Debug, Args)]
pub struct GenbankToFastaArgs {
    ///genbank file
    #[arg(short, long, value_name = "IN_GENBANK_FILE")]
    pub genbank_file: Option<String>,

    ///output fasta file
    #[arg(short, long, value_name = "OUT_FASTA_FILE")]
    output_fasta_file: Option<String>,
}

/// .
#[doc = r"Convert GENBANK to FASTA format"]
pub fn run(options: GenbankToFastaArgs) {
    let genbank_file;

    match options.genbank_file {
        Some(filename) => {
            genbank_file = File::open(filename).unwrap();
        }
        None => {
            genbank_file = File::open("/dev/stdin").unwrap();
        }
    }

    let mut gz_encoder;
    let mut fasta_writer: BufWriter<Box<dyn Write>> = match options.output_fasta_file {
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

    for seq in SeqReader::new(genbank_file) {
        let seq1 = seq.unwrap();

        let acc = seq1.accession.unwrap_or(String::from("default"));
        writeln!(fasta_writer, ">{}", acc).expect("Cannot write out ID");

        for c in seq1.seq {
            match c {
                b'a' => write!(fasta_writer, "A").expect("Cannot write out sequence"),
                b'c' => write!(fasta_writer, "C").expect("Cannot write out sequence"),
                b'g' => write!(fasta_writer, "G").expect("Cannot write out sequence"),
                b't' => write!(fasta_writer, "T").expect("Cannot write out sequence"),
                _ => write!(fasta_writer, "N").expect("Cannot write out sequence"),
            }
        }

        write!(fasta_writer, "\n").expect("Cannot write out sequence");
    }
}
