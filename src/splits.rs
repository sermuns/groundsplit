use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Splits {
    pub title: String,
    pub splits: Vec<Split>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Split {
    pub name: String,
    pub ms_since_start: u128,
}

impl From<(&str, u128)> for Split {
    fn from((name_str, ms_since_start): (&str, u128)) -> Self {
        Self {
            name: name_str.to_owned(),
            ms_since_start,
        }
    }
}
