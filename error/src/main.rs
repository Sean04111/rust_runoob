// this is basically about the errors in rust =D

// recoverable error & unrecoerable error
// result & option

mod result;
use result::result_basic;

mod error;
use error::error_basic;
fn main() {
    //result_basic();
    error_basic();
}
