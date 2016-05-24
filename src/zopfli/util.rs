use libc::{size_t, c_double};

pub const ZOPFLI_NUM_LL: size_t = 288;
pub const ZOPFLI_NUM_D: size_t = 32;

pub const ZOPFLI_WINDOW_SIZE: size_t = 32768;
pub const ZOPFLI_WINDOW_MASK: size_t = 32767; // ZOPFLI_WINDOW_SIZE - 1
pub const ZOPFLI_MAX_MATCH: size_t = 258;
pub const ZOPFLI_MIN_MATCH: size_t = 3;
pub const ZOPFLI_CACHE_LENGTH: size_t = 8;
pub const ZOPFLI_MAX_CHAIN_HITS: size_t = 8192;
pub const ZOPFLI_LARGE_FLOAT: c_double = 1E30;
