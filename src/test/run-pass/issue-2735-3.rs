// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This test should behave exactly like issue-2735-2
struct defer<'self> {
    b: &'self mut bool,
}

#[unsafe_destructor]
impl<'self> Drop for defer<'self> {
    fn finalize(&self) {
        unsafe {
            *(self.b) = true;
        }
    }
}

fn defer<'r>(b: &'r mut bool) -> defer<'r> {
    defer {
        b: b
    }
}

pub fn main() {
    let mut dtor_ran = false;
    defer(&mut dtor_ran);
    assert!((dtor_ran));
}
