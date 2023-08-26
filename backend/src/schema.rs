pub mod stocks;
pub mod quotes;
pub mod trades;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Pagination {
    #[serde(default="default_page")]
    pub page: i64,
    #[serde(default="default_page_size")]
    pub page_size: i64,
}
fn default_page_size() -> i64 {
    100
}
fn default_page() -> i64 {
    0
}