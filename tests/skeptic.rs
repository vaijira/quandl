#[cfg(all(feature = "skeptic", feature = "test-quandl-api"))]
include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
