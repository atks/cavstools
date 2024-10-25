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
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

mod annotate;
mod args;
mod filter;
mod genbanktofasta;
mod stats;
mod view;
use args::FtArgs;
use atty::Stream;
use clap::Parser;

fn main() {
    let args: FtArgs = FtArgs::parse();

    let mut in_pipe = true;
    if atty::is(Stream::Stdin) || atty::is(Stream::Stdout) {
        // eprintln!("not used in a pipe");
        in_pipe = false;
    } else {
        // eprintln!("used in a pipe");
    }

    match args.entity_type {
        args::EntityType::View(args) => view::run(args),
        args::EntityType::Filter(args) => filter::run(args),
        args::EntityType::Stats(args) => stats::run(args),
        args::EntityType::GenbankToFasta(args) => genbanktofasta::run(args),
        args::EntityType::Annotate(args) => annotate::run(args),
    }
}
