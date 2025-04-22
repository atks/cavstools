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
use std::fs::File;

#[derive(Debug, Args)]
pub struct AnnotateArgs {
    ///genbank file
    #[arg(short, long, value_name = "IN_GENBANK_FILE")]
    pub genbank_file: Option<String>,

    ///input fasta file
    #[arg(short, long, value_name = "IN_FASTA_FILE")]
    input_fasta_file: Option<String>,
}

/// Annotate a fasta file with features from a genbank file.
///

pub fn run(args: AnnotateArgs) {
    let genbank_file = File::open(args.genbank_file.unwrap()).unwrap();
    let input_fasta_file = File::open(args.input_fasta_file.unwrap()).unwrap();
    // let reader = needletail::parse_genbank(genbank_file).unwrap();
    // let mut writer = needletail::parse_fasta(input_fasta_file).unwrap();
    // for record in reader {
    //     let record = record.unwrap();
    //     writer.write_record(&record).unwrap();
    // }
}
