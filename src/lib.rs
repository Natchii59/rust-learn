use std::collections::*;
use std::fmt::Result;
use std::io::{self, Result as IoResult};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        super::add_to_waitlist();
    }
}

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
