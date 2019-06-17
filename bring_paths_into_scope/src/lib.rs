mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// same as, however usage of "self" should be going away
//use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// two items with the same name

use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}


// can use as

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {}
fn function4() -> IoResult<()> {}


// instead of using
use std::cmp::Ordering;
use std::io;

// use

use std::{cmp::Ordering, io};

// if
use std::io;
use std::io::Write;

// can use 
use std::io{self, Write};

// bring in everything public using the glob operator
use std::collections::*;