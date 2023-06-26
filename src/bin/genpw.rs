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

use clap::Parser;
use rand::prelude::*;
use rand::{
    distributions::Alphanumeric, distributions::Uniform, distributions::WeightedIndex, Rng,
}; // 0.8

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Mode of password generation
    #[clap(short, long, value_parser, default_value = "wtd")]
    mode: String,

    /// Number of 4 tuples to use for password
    #[clap(short, long, value_parser, default_value_t = 4)]
    c: usize,
}

fn main() {
    let args = Args::parse();

    //println!("mode: {}", args.mode);
    //println!("c:    {}", args.c);

    if args.mode == "uni" {
        let mut ks = Vec::new();

        for _i in 1..=args.c {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(4)
                .map(char::from)
                .collect();

            ks.push(s.to_string());
        }

        let fs = ks.join("-");

        println!("{}", fs);
    } else if args.mode == "std" {
        let mut rng = rand::thread_rng();
        let side = Uniform::new(1, 5);

        let (no_cap, no_num) = (rng.sample(side), rng.sample(side));
        //println!("Categories: {}, {}", no_cap, no_num);

        let lc = Uniform::new(97, 123);
        let uc = Uniform::new(65, 91);
        let nu = Uniform::new(48, 57);

        let s_lc: String = rand::thread_rng()
            .sample_iter(&lc)
            .take(args.c * 4 - no_cap - no_num)
            .map(char::from)
            .collect();

        let s_uc: String = rand::thread_rng()
            .sample_iter(&uc)
            .take(no_cap)
            .map(char::from)
            .collect();

        let s_nu: String = rand::thread_rng()
            .sample_iter(&nu)
            .take(no_num)
            .map(char::from)
            .collect();

        //println!("{}-{}-{}", s_lc, s_uc, s_nu);

        let mut s: String = "".to_string();

        s.push_str(&s_lc);
        s.push_str(&s_uc);
        s.push_str(&s_nu);

        //println!("{}", s);

        let shuf = Uniform::new(0, s.len());
        let mut chars: Vec<_> = s.chars().collect();

        for _ in 0..1000 {
            let (i, j) = (rng.sample(shuf), rng.sample(shuf));
            chars.swap(i, j);
            //println!("{} {}", i, j);
            //let c = s[j];
            //s[j] = s[i];
        }

        let ns: String = chars.into_iter().collect();
        //println!("{}", ns);

        let chars: Vec<_> = ns.chars().collect();
        let mut chars1: Vec<_> = s.chars().collect();
        chars1.clear();
        for i in 0..chars.len() {
            chars1.push(chars[i]);
            if i % 4 == 3 && i != (chars.len() - 1) {
                chars1.push('-');
            }
        }

        let fs: String = chars1.into_iter().collect();
        println!("{}", fs);
    } else {
        let mut rng = rand::thread_rng();
        let side = Uniform::new(1, 2);
        let side2 = Uniform::new(1, 5);

        let (no_cap, no_num, no_con) = (rng.sample(side), rng.sample(side), rng.sample(side2));
        //println!("Categories: {}, {}", no_cap, no_num);

        let lc = Uniform::new(97, 123);
        let uc = Uniform::new(65, 91);
        let nu = Uniform::new(48, 57);

        let s_lc: String = rand::thread_rng()
            .sample_iter(&lc)
            .take(no_con)
            .map(char::from)
            .collect();

        let s_uc: String = rand::thread_rng()
            .sample_iter(&uc)
            .take(no_cap)
            .map(char::from)
            .collect();

        let s_nu: String = rand::thread_rng()
            .sample_iter(&nu)
            .take(no_num)
            .map(char::from)
            .collect();

        let mut s: String = "".to_string();

        s.push_str(&s_lc);
        s.push_str(&s_uc);
        s.push_str(&s_nu);

        let choices = ['a', 'e', 'i', 'o', 'u'];
        let weights = [2, 2, 1, 2, 1];
        let dist = WeightedIndex::new(&weights).unwrap();
        for _ in 0..(args.c * 4 - no_cap - no_num - no_con) {
            s.push(choices[dist.sample(&mut rng)]);
        }

        //println!("{}", s);

        let shuf = Uniform::new(0, s.len());
        let mut chars: Vec<_> = s.chars().collect();

        for _ in 0..1000 {
            let (i, j) = (rng.sample(shuf), rng.sample(shuf));
            chars.swap(i, j);
            //println!("{} {}", i, j);
            //let c = s[j];
            //s[j] = s[i];
        }

        let ns: String = chars.into_iter().collect();
        //println!("{}", ns);

        let chars: Vec<_> = ns.chars().collect();
        let mut chars1: Vec<_> = s.chars().collect();
        chars1.clear();
        for i in 0..chars.len() {
            chars1.push(chars[i]);
            if i % 4 == 3 && i != (chars.len() - 1) {
                chars1.push('-');
            }
        }

        let fs: String = chars1.into_iter().collect();
        println!("{}", fs);
    }
}
