// The Computer Language Benchmarks Game
// http://benchmarksgame.alioth.debian.org/
//
// contributed by the Rust Project Developers

// Copyright (c) 2013-2014 The Rust Project Developers
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// - Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
// - Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in
//   the documentation and/or other materials provided with the
//   distribution.
//
// - Neither the name of "The Computer Language Benchmarks Game" nor
//   the name of "The Computer Language Shootout Benchmarks" nor the
//   names of its contributors may be used to endorse or promote
//   products derived from this software without specific prior
//   written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate num;
extern crate test;

use std::str::FromStr;
use std::num::{FromPrimitive, ToPrimitive};

use test::Bencher;

use num::{BigInt, Integer, One, Zero};

struct Context {
    numer: BigInt,
    accum: BigInt,
    denom: BigInt,
}

impl Context {
    fn new() -> Context {
        Context {
            numer: One::one(),
            accum: Zero::zero(),
            denom: One::one(),
        }
    }

    fn from_i32(i: i32) -> BigInt {
        FromPrimitive::from_i32(i).unwrap()
    }

    fn extract_digit(&self) -> i32 {
        if self.numer > self.accum {return -1;}
        let (q, r) =
            (&self.numer * Context::from_i32(3) + &self.accum)
            .div_rem(&self.denom);
        if r + &self.numer >= self.denom {return -1;}
        q.to_i32().unwrap()
    }

    fn next_term(&mut self, k: i32) {
        let y2 = Context::from_i32(k * 2 + 1);
        self.accum = (&self.accum + (&self.numer << 1)) * &y2;
        self.numer = &self.numer * Context::from_i32(k);
        self.denom = &self.denom * y2;
    }

    fn eliminate_digit(&mut self, d: i32) {
        let d = Context::from_i32(d);
        let ten = Context::from_i32(10);
        self.accum = (&self.accum - &self.denom * d) * &ten;
        self.numer = &self.numer * ten;
    }
}

fn pidigits(n: isize) {
    let mut k = 0;
    let mut context = Context::new();

    for i in (1..(n+1)) {
        let mut d;
        loop {
            k += 1;
            context.next_term(k);
            d = context.extract_digit();
            if d != -1 {break;}
        }

        print!("{}", d);
        if i % 10 == 0 {print!("\t:{}\n", i);}

        context.eliminate_digit(d);
    }

    let m = n % 10;
    if m != 0 {
        for _ in (m..10) { print!(" "); }
        print!("\t:{}\n", n);
    }
}

static DEFAULT_DIGITS: isize = 512;

#[bench]
fn use_bencher(b: &mut Bencher) {
    b.iter(|| pidigits(DEFAULT_DIGITS))
}

fn main() {
    let args = std::os::args();
    let args = args.as_slice();
    let n = if args.len() < 2 {
        DEFAULT_DIGITS
    } else {
        FromStr::from_str(args[1].as_slice()).unwrap()
    };
    pidigits(n);
}
