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

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use clap::Parser;
use gb_io::reader::SeqReader;
use std::fs::File;
use std::io;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "gb2fasta - converts genbank format to fasta"
)]
struct Args {
    /// Genbank File
    #[clap(value_parser)]
    gb_file: Option<String>,
}

fn main() {
    #[allow(unused_variables, unused_mut, dead_code)]
    let args = Args::parse();

    println!("name: {:?}", args.gb_file.as_deref());

    #[allow(unused_variables, unused_mut, dead_code)]
    if args.gb_file != None {
        let file = File::open(args.gb_file.unwrap()).unwrap();
        let stdout = io::stdout();
        for seq in SeqReader::new(file) {
            let seq = seq.unwrap();
            //let rc = seq.revcomp();
            //println!(seq.seq);

            let sequence = seq.extract_range_seq(0, 14);
            print_type_of(&sequence);
            print_type_of(&seq.accession);

            let fs: String = "sdsd".to_string();
            print_type_of(&fs);
            println!("acc {}", seq.accession.unwrap());
            println!("len {}", seq.len.unwrap());
            //ASCII lowercase encoding
            //a-z 97-122
            //97 A
            //99 C
            //103 G
            //116 T
            for _i in 0..14 {
                // println!("{}", sequence[i]);
            }

            //rc.write(stdout.lock()).unwrap();
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
