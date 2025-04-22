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
use gb_io::reader::SeqReader;
use gb_io::seq::Location;
use std::fs::File;
use string_interner::StringInterner;

#[derive(Debug, Args)]
pub struct AnnotateArgs {
    ///genbank file
    #[arg(short, long, value_name = "IN_GENBANK_FILE")]
    pub genbank_file: Option<String>,

    ///input fasta file
    #[arg(short, long, value_name = "IN_FASTA_FILE")]
    input_fasta_file: Option<String>,
}

/// .
#[doc = r"Annotate a sequence using a reference genbank record"]
pub fn run(options: AnnotateArgs) {
    //process arguments
    println!("Annotating sequence");

    let genbank_file;

    match options.genbank_file {
        Some(filename) => {
            genbank_file = File::open(filename).unwrap();
        }
        None => {
            genbank_file = File::open("/dev/stdin").unwrap();
        }
    }

    let mut interner = StringInterner::default();
    //interned strings
    let gene = interner.get_or_intern("gene");

    //read genbank file
    //we only expect one sequence entry in the genbank file
    for seq in SeqReader::new(genbank_file) {
        let seq1 = seq.unwrap();

        let acc = seq1.accession.unwrap_or(String::from("default"));
        println!("accession: {}", acc);

        //process features
        let mut no_features = 0;
        for feature in seq1.features {
            println!("kind: {:?}", feature.kind);
            println!("location: {:?}", feature.location);
            match feature.location {
                Location::Range(x, y) => {
                    println!("range: {:?} {:?}", x, y);
                    println!("range: {:?} {:?}", x.0, y.0);
                }
                Location::OneOf(one_of) => {
                    println!("one_of: {:?}", one_of);
                }
                Location::Between(x, y) => {
                    println!("between: {:?} {:?}", x, y);
                }
                Location::Complement(x) => {
                    println!("complement: {}", *x);
                    println!("complement: {}", x);
                }
                Location::Join(join) => {
                    println!("join: {:?}", join);
                }
                Location::Order(order) => {
                    println!("order: {:?}", order);
                }
                Location::Bond(order) => {
                    println!("bond: {:?}", order);
                }
                Location::External(x, y) => {
                    println!("external: {:?} {:?}", x, y);
                }
                Location::Gap(gap) => {
                    println!("gap: {:?}", gap);
                }
                _ => println!("Not handled!"),
            }
            for qualifier in feature.qualifiers {
                match qualifier {
                    (x, y) => {
                        println!("x: {:?}", x);
                        println!("y: {:?}", y);
                    }
                    // gb_io::seq::QualifierKey::Allele(allele) => {
                    //     println!("\tallele: {:?}", allele);
                    // }
                    // gb_io::record::Qualifier::Citation(citation) => {
                    //     println!("\tcitation: {:?}", citation);
                    // }
                    // gb_io::record::Qualifier::Codon(codon) => {
                    //     println!("\tcodon: {:?}", codon);
                    // }
                    // gb_io::record::Qualifier::Compare(compare) => {
                    //     println!("\tcompare: {:?}", compare);
                    // }
                    // gb_io::record::Qualifier::Country(country) => {
                    //     println!("\tcountry: {:?}", country);
                    // }
                    // gb_io::record::Qualifier::DbXref(dbxref) => {
                    //     println!("\tdbxref: {:?}", dbxref);
                    // }
                    // gb_io::record::Qualifier::Direction(direction) => {
                    //     println!("\tdirection: {:?}", direction);
                    // }
                    // gb_io::record::Qualifier::EC_number(ec_number) => {
                    //     println!("\tec_number: {:?}", ec_number);
                    // }
                    // gb_io::record::Qualifier::Experiment(experiment) => {
                    //     println!("\texperiment: {:?}", experiment);
                    // }
                    // gb_io::record::Qualifier::Function(function) => {
                    //     println!("\tfunction: {:?}", function);
                    // }
                    // gb_io::seq::QualifierKeyStatisticSet::Gene(gene) => {
                    //     println!("\tgene: {:?}", gene);
                    // }
                    // gb_io::record::Qualifier::GenMapDbXref(gen_map_dbxref) => {
                    //     println!("\tgen_map_dbxref: {:?}", gen_map_dbxref);
                    // }
                    // gb_io::record::Qualifier::Inference(inference) => {
                    //     println!("\tinference: {:?}", inference);
                    // }
                    // gb_io::record::Qualifier::Isolate(isolate) => {
                    //     println!("\tisolate: {:?}", isolate);
                    // }
                    // gb_io::record::Qualifier::Label(label) => {
                    //     println!("\tlabel: {:?}", label);
                    // }
                    // gb_io::record::Qualifier::LocusTag(locus_tag) => {
                    //     println!("\tlocus_tag: {:?}", locus_tag);
                    // }
                    // gb_io::record::Qualifier::Map(map) => {
                    //     println!("\tmap: {:?}", map);
                    // }
                    // gb_io::record::Qualifier::MobileElement(mobile_element) => {
                    //     println!("\tmobile_element: {:?}", mobile_element);
                    // }
                    _ => println!("\tqualifier: {:?}", qualifier),
                }
            }
            println!("\n\n");
            no_features += 1;
        }

        println!("no of features: {}", no_features);
    }

    //read fasta file

    //annotate
    //blast features to reference
    //check for frameshifts, nonsense mutations, etc etc
    //transfer annotations

    //write annotated fasta file
}
