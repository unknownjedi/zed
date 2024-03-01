pub mod completion;
pub mod embedding;
pub mod model;

pub use completion::*;
pub use embedding::*;
pub use model::OpenAiLanguageModel;

pub const OPEN_AI_API_URL: &'static str = "https://api.openai.com/v1";
pub const OPEN_AI_API_VERSION: &'static str = "2023-07-01-preview";
