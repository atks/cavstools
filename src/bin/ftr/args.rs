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

use crate::annotate::AnnotateArgs;
use crate::filter::FilterArgs;
use crate::genbanktofasta::GenbankToFastaArgs;
use crate::stats::StatsArgs;
use crate::view::ViewArgs;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FtArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    ///View FASTA/Q file
    View(ViewArgs),
    ///Filter sequences
    Filter(FilterArgs),
    ///Compute sequence statistics
    ///
    ///    read_id, avg_quality_score, length
    Stats(StatsArgs),
    ///Convert genbank format to FASTA
    GenbankToFasta(GenbankToFastaArgs),
    ///Annotate genbank features on a new sequence test
    Annotate(AnnotateArgs),
}
