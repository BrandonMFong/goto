/**
 * author: brando
 * date: 10/24/24
 */

use std::env;

pub struct Args {
    args: Vec<String>
}

impl Args {
    pub fn new() -> Args {
        Args {
            args: env::args().collect()
        }
    }
}

