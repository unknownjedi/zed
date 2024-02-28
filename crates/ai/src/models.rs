use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub enum TruncationDirection {
    Start,
    End,
}

pub trait LanguageModel {
    fn name(&self) -> String;
    fn count_tokens(&self, content: &str) -> anyhow::Result<usize>;
    fn truncate(
        &self,
        content: &str,
        length: usize,
        direction: TruncationDirection,
    ) -> anyhow::Result<String>;
    fn capacity(&self) -> anyhow::Result<usize>;
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelEndpoint {
    #[serde(rename = "openai")]
    #[default]
    Openai,
    #[serde(rename = "azure")]
    Azure,
}
