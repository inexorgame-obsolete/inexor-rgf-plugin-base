use async_trait::async_trait;
use log::debug;
use log::error;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::FlowType;
use crate::plugins::FlowTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/flows"]
struct BaseFlowTypeAsset;

#[async_trait]
pub trait BaseFlowTypeProvider: FlowTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct BaseFlowTypeProviderImpl {}

interfaces!(BaseFlowTypeProviderImpl: dyn FlowTypeProvider);

#[component]
impl BaseFlowTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl BaseFlowTypeProvider for BaseFlowTypeProviderImpl {}

impl FlowTypeProvider for BaseFlowTypeProviderImpl {
    fn get_flow_types(&self) -> Vec<FlowType> {
        let mut flow_types = Vec::new();
        for file in BaseFlowTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading flow_type from resource {}", filename);
            let asset = BaseFlowTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let flow_type: FlowType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(flow_type) => flow_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            flow_types.push(flow_type);
        }
        flow_types
    }
}
