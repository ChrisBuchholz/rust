// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::Debug;
use std::default::Default;

// Test that a blank impl for all T conflicts with an impl for some
// specific T.

trait MyTrait {
    fn get(&self) -> usize;
}

impl<T> MyTrait for T { //~ ERROR E0119
    fn get(&self) -> usize { 0 }
}

struct MyType {
    dummy: usize
}

impl MyTrait for MyType {
    fn get(&self) -> usize { self.dummy }
}

fn main() { }
