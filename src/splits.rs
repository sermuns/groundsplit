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

impl Split {
    // TODO: less alloc?
    pub fn duration_since_start_str(&self) -> String {
        let &Self { ms_since_start, .. } = self;

        let secs_since_start = ms_since_start / 1_000;
        let minutes = secs_since_start / 60;
        let seconds = secs_since_start % 60;

        // TODO:
        // support more timestamp styles,
        // like with hour (0:00:00)
        format!("{:01}:{:02}", minutes, seconds,)
    }
}
