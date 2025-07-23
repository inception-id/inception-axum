use regex::Regex;
use std::sync::LazyLock;

pub static RE_PHONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^8\d*$").unwrap());
pub static RE_COMPANY_PHONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d*$").unwrap());
