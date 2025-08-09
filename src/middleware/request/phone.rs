use regex::Regex;
use std::sync::LazyLock;

pub static RE_PHONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^08\d*$").unwrap());
