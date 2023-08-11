use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ControlMessage {
    #[serde(rename(deserialize = "T"))]
    status: String,
    code: i16,
    msg: String,
}

#[derive(Serialize, Default, Debug)]
pub struct ActionMessage {
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trades: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quotes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bars: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "dailyBars")
    )]
    daily_bars: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "updatedBars")
    )]
    updated_bars: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statuses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lulds: Option<Vec<String>>,
}

impl ActionMessage {
    pub fn new(action: &str) -> Self {
        let mut msg = ActionMessage::default();
        msg.action = action.to_string();

        msg
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = Some(key.to_string());

        self
    }

    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(secret.to_string());

        self
    }

    pub fn trades(mut self, trades: &[&str]) -> Self {
        self.trades = Some(trades.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn quotes(mut self, quotes: &[&str]) -> Self {
        self.quotes = Some(quotes.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn bars(mut self, bars: &[&str]) -> Self {
        self.bars = Some(bars.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn daily_bars(mut self, daily_bars: &[&str]) -> Self {
        self.daily_bars = Some(daily_bars.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn updated_bars(mut self, updated_bars: &[&str]) -> Self {
        self.updated_bars = Some(updated_bars.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn statuses(mut self, statuses: &[&str]) -> Self {
        self.statuses = Some(statuses.iter().map(|&s| s.to_string()).collect());

        self
    }

    pub fn lulds(mut self, lulds: &[&str]) -> Self {
        self.lulds = Some(lulds.iter().map(|&s| s.to_string()).collect());

        self
    }
}
