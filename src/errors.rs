//! Error handling.

use std::io;

error_chain!{
    // doc attributes are required to workaround
    // https://github.com/rust-lang-nursery/error-chain/issues/63
    foreign_links {
        Io(io::Error) #[doc = "I/O error."];
    }
}
