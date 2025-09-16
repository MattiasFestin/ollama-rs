#[cfg_attr(docsrs, doc(cfg(feature = "tool-implementations")))]
#[cfg(feature = "tool-implementations")]
pub mod implementations;

use std::{future::Future, pin::Pin};

use ollama_rs_types::{OllamaResult, Parameters, Tool};
use schemars::{r#gen::SchemaSettings, schema::RootSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;


/// It's highly recommended that the `JsonSchema` has descriptions for all attributes.
/// Descriptions can be defined with `#[schemars(description = "Hi I am an attribute")]` above each attribute
// TODO enforce at compile-time

pub trait ToolHolder: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn call(
        &mut self,
        parameters: Value,
    ) -> Pin<Box<dyn Future<Output = OllamaResult<String>> + '_ + Send>>;
}

impl<T: Tool> ToolHolder for T {
    fn name(&self) -> String {
        <T as Tool>::name().to_string()
    }
    fn description(&self) -> String {
        <T as Tool>::description().to_string()
    }
    fn call(
        &mut self,
        parameters: Value,
    ) -> Pin<Box<dyn Future<Output = OllamaResult<String>> + '_ + Send>> {
        
        Box::pin(async move {
            let parameters = match serde_json::from_value(parameters) {
                Ok(params) => params,
                Err(_) => return Err("Failed to deserialize parameters".into()),
            };

            T::call(self, parameters).await
        })
    }
}

