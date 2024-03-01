use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

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
    OpenAi,
    Azure,
}

impl ModelEndpoint {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => ModelEndpoint::OpenAi,
            "azure" => ModelEndpoint::Azure,
            _ => ModelEndpoint::default(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ModelEndpoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(ModelEndpoint::from_str(&s))
    }
}
