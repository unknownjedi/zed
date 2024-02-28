use ai::models::ModelEndpoint;
use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum OpenAiModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-1106-preview")]
    FourTurbo,
    Custom,
}

impl OpenAiModel {
    pub fn from_name(model_name: &str) -> OpenAiModel {
        match model_name {
            "gpt-3.5-turbo-0613" => OpenAiModel::ThreePointFiveTurbo,
            "gpt-4-0613" => OpenAiModel::Four,
            "gpt-4-1106-preview" => OpenAiModel::FourTurbo,
            _ => OpenAiModel::Custom,
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            OpenAiModel::ThreePointFiveTurbo => "gpt-3.5-turbo-0613",
            OpenAiModel::Four => "gpt-4-0613",
            OpenAiModel::FourTurbo => "gpt-4-1106-preview",
            OpenAiModel::Custom => "Custom",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            OpenAiModel::ThreePointFiveTurbo => "gpt-3.5-turbo",
            OpenAiModel::Four => "gpt-4",
            OpenAiModel::FourTurbo => "gpt-4-turbo",
            OpenAiModel::Custom => "Custom",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            OpenAiModel::ThreePointFiveTurbo => OpenAiModel::Four,
            OpenAiModel::Four => OpenAiModel::FourTurbo,
            OpenAiModel::FourTurbo => OpenAiModel::Custom,
            OpenAiModel::Custom => OpenAiModel::ThreePointFiveTurbo,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    Right,
    Bottom,
}

#[derive(Deserialize, Debug)]
pub struct AssistantSettings {
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub model_name: String,
    pub endpoint: ModelEndpoint,
    pub endpoint_url: String,
    pub api_version: String,
}

/// Assistant panel settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    pub dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<f32>,
    /// The default OpenAI model to use when starting new conversations.
    ///
    /// Default: gpt-4-1106-preview
    pub model_name: Option<String>,
    /// OpenAI API base URL to use when starting new conversations.
    ///
    /// Default: https://api.openai.com/v1
    pub endpoint_url: Option<String>,
    /// OpenAI or AZURE.
    ///
    /// Default: OPENAI
    pub endpoint: Option<ModelEndpoint>,
    /// Api version when endpoint is AZURE
    ///
    /// Default: 2023-07-01-preview
    pub api_version: Option<String>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
