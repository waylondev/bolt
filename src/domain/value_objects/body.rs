use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    content: String,
    content_type: ContentType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    Json,
    Text,
    FormUrlEncoded,
    FormData,
    Xml,
    Binary,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            content: String::new(),
            content_type: ContentType::Text,
        }
    }
}

impl Body {
    pub fn json(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            content_type: ContentType::Json,
        }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            content_type: ContentType::Text,
        }
    }

    pub fn from_string(content: impl Into<String>, content_type: ContentType) -> Self {
        Self {
            content: content.into(),
            content_type,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_type(&self) -> ContentType {
        self.content_type
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

use std::fmt;

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
