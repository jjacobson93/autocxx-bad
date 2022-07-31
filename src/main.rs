use autocxx::prelude::*;

include_cpp! {
    #include "bad.hpp"
    safety!(unsafe)
    generate!("nextWchar")
}

fn main() {}
