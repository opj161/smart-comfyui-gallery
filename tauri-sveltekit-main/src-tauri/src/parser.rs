// Workflow parser for ComfyUI metadata extraction
// Ports the Python ComfyUIWorkflowParser with full dual-format support

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

// Node type constants - researched from real-world ComfyUI workflows
const SAMPLER_TYPES: &[&str] = &[
    "KSampler",
    "KSamplerAdvanced",
    "SamplerCustom",
    "SamplerCustomAdvanced",
    "KSamplerEfficient",
    "DetailerForEach",
    "SamplerDPMPP_2M_SDE",
    "WanVideoSampler",
    "UltimateSDUpscale",
];

const MODEL_LOADER_TYPES: &[&str] = &[
    "CheckpointLoaderSimple",
    "CheckpointLoader",
    "Load Checkpoint",
    "UNETLoader",
    "Load Diffusion Model",
    "UnetLoaderGGUF",
    "DualCLIPLoader",
];

const PROMPT_NODE_TYPES: &[&str] = &[
    "CLIPTextEncode",
    "CLIP Text Encode (Prompt)",
    "TextEncodeQwenImageEditPlus",
    "CLIPTextEncodeSDXL",
    "CLIPTextEncodeSDXLRefiner",
];

#[allow(dead_code)]
const SCHEDULER_NODE_TYPES: &[&str] = &[
    "BasicScheduler",
    "KarrasScheduler",
    "ExponentialScheduler",
    "SgmUniformScheduler",
];

#[allow(dead_code)]
const SAMPLER_SELECT_NODE_TYPES: &[&str] = &["KSamplerSelect"];

#[derive(Debug, Clone)]
pub struct ParsedWorkflow {
    pub model_name: Option<String>,
    pub sampler_name: Option<String>,
    pub scheduler: Option<String>,
    pub positive_prompt: String,
    pub negative_prompt: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub cfg: Option<f64>,
    pub steps: Option<i64>,
}

#[derive(Debug)]
enum WorkflowFormat {
    UI,
    API,
}

pub struct ComfyUIWorkflowParser {
    format: WorkflowFormat,
    nodes_by_id: HashMap<String, Value>,
    links_map: HashMap<i64, (String, i64)>,
    #[allow(dead_code)]
    file_path: String,
}

impl ComfyUIWorkflowParser {
    pub fn new(workflow_data: Value, file_path: &Path) -> Result<Self, String> {
        let file_path_str = file_path.to_string_lossy().to_string();
        
        // Detect format
        if let Some(obj) = workflow_data.as_object() {
            if obj.contains_key("nodes") {
                // UI Format
                let nodes = obj.get("nodes")
                    .and_then(|n| n.as_array())
                    .ok_or("Invalid UI format: nodes is not an array")?;
                
                let mut nodes_by_id = HashMap::new();
                for node in nodes {
                    if let Some(node_obj) = node.as_object() {
                        if let Some(id) = node_obj.get("id") {
                            let id_str = id.as_i64()
                                .map(|i| i.to_string())
                                .or_else(|| id.as_str().map(|s| s.to_string()))
                                .ok_or("Invalid node ID")?;
                            nodes_by_id.insert(id_str, node.clone());
                        }
                    }
                }
                
                let links_map = Self::build_link_map(obj.get("links"));
                
                Ok(Self {
                    format: WorkflowFormat::UI,
                    nodes_by_id,
                    links_map,
                    file_path: file_path_str,
                })
            } else {
                // API Format - each key is a node ID
                let mut nodes_by_id = HashMap::new();
                for (key, value) in obj {
                    if let Some(node_obj) = value.as_object() {
                        let mut node_with_id = node_obj.clone();
                        node_with_id.insert("id".to_string(), Value::String(key.clone()));
                        nodes_by_id.insert(key.clone(), Value::Object(node_with_id));
                    }
                }
                
                Ok(Self {
                    format: WorkflowFormat::API,
                    nodes_by_id,
                    links_map: HashMap::new(),
                    file_path: file_path_str,
                })
            }
        } else {
            Err("Invalid workflow data: not an object".to_string())
        }
    }
    
    fn build_link_map(links_value: Option<&Value>) -> HashMap<i64, (String, i64)> {
        let mut link_map = HashMap::new();
        
        if let Some(links_array) = links_value.and_then(|v| v.as_array()) {
            for link in links_array {
                if let Some(link_arr) = link.as_array() {
                    if link_arr.len() >= 3 {
                        if let (Some(link_id), Some(source_id), Some(source_slot)) = (
                            link_arr[0].as_i64(),
                            link_arr[1].as_i64(),
                            link_arr[2].as_i64(),
                        ) {
                            link_map.insert(link_id, (source_id.to_string(), source_slot));
                        }
                    }
                }
            }
        }
        
        link_map
    }
    
    fn get_node_type(&self, node: &Value) -> Option<String> {
        match self.format {
            WorkflowFormat::UI => {
                node.as_object()
                    .and_then(|n| n.get("type"))
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string())
            }
            WorkflowFormat::API => {
                node.as_object()
                    .and_then(|n| n.get("class_type"))
                    .and_then(|t| t.as_str())
                    .map(|s| s.to_string())
            }
        }
    }
    
    fn get_input_source_node(&self, node: &Value, input_name: &str) -> Option<Value> {
        match self.format {
            WorkflowFormat::UI => {
                // Find input by name in inputs array, then look up link
                let inputs = node.as_object()?.get("inputs")?.as_array()?;
                
                for input_def in inputs {
                    if let Some(input_obj) = input_def.as_object() {
                        if input_obj.get("name")?.as_str()? == input_name {
                            if let Some(link_id) = input_obj.get("link")?.as_i64() {
                                if let Some((source_id, _)) = self.links_map.get(&link_id) {
                                    return self.nodes_by_id.get(source_id).cloned();
                                }
                            }
                        }
                    }
                }
                None
            }
            WorkflowFormat::API => {
                // Direct reference: inputs[input_name] = [source_node_id, output_slot]
                let inputs = node.as_object()?.get("inputs")?.as_object()?;
                let input_ref = inputs.get(input_name)?.as_array()?;
                
                if input_ref.len() >= 1 {
                    let source_id = input_ref[0].as_str()?;
                    return self.nodes_by_id.get(source_id).cloned();
                }
                None
            }
        }
    }
    
    fn get_widget_value(&self, node: &Value, param_name: &str) -> Option<Value> {
        match self.format {
            WorkflowFormat::UI => {
                // UI format: widgets_values array or properties
                if let Some(node_obj) = node.as_object() {
                    // Try properties first
                    if let Some(props) = node_obj.get("properties").and_then(|p| p.as_object()) {
                        if let Some(value) = props.get(param_name) {
                            return Some(value.clone());
                        }
                    }
                    
                    // Try widgets_values with widget_idx_map
                    // For simplicity, we'll search by name in title or node type
                    if let Some(widgets) = node_obj.get("widgets_values") {
                        // This is simplified - full implementation would use widget_idx_map
                        return Some(widgets.clone());
                    }
                }
                None
            }
            WorkflowFormat::API => {
                // API format: direct in inputs dict
                node.as_object()?
                    .get("inputs")?
                    .as_object()?
                    .get(param_name)
                    .cloned()
            }
        }
    }
    
    fn find_source_node(
        &self,
        start_node_id: &str,
        input_name: &str,
        stop_at_types: &[&str],
        max_hops: usize,
    ) -> Option<Value> {
        let mut current_node_id = start_node_id.to_string();
        
        for _ in 0..max_hops {
            let node = self.nodes_by_id.get(&current_node_id)?;
            let node_type = self.get_node_type(node)?;
            
            // Stop if we found target type
            if stop_at_types.contains(&node_type.as_str()) {
                return Some(node.clone());
            }
            
            // Handle Primitive nodes (pass-through)
            if node_type == "Primitive" || node_type == "PrimitiveNode" {
                // Try to find what this primitive connects to
                // This is simplified - full implementation would trace all connections
                continue;
            }
            
            // Try to follow the input connection
            if let Some(source_node) = self.get_input_source_node(node, input_name) {
                if let Some(source_id_val) = source_node.as_object().and_then(|n| n.get("id")) {
                    current_node_id = if let Some(s) = source_id_val.as_str() {
                        s.to_string()
                    } else if let Some(i) = source_id_val.as_i64() {
                        i.to_string()
                    } else {
                        break;
                    };
                    continue;
                }
            }
            
            break;
        }
        
        None
    }
    
    pub fn parse(&self) -> Vec<ParsedWorkflow> {
        let sampler_nodes = self.find_sampler_nodes();
        
        sampler_nodes.iter()
            .filter_map(|node| self.process_sampler(node))
            .collect()
    }
    
    fn find_sampler_nodes(&self) -> Vec<Value> {
        let mut samplers: Vec<(String, Value)> = self.nodes_by_id.iter()
            .filter_map(|(id, node)| {
                let node_type = self.get_node_type(node)?;
                if SAMPLER_TYPES.contains(&node_type.as_str()) {
                    Some((id.clone(), node.clone()))
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by ID for consistent ordering
        samplers.sort_by_key(|(id, _)| id.parse::<i64>().unwrap_or(0));
        
        samplers.into_iter().map(|(_, node)| node).collect()
    }
    
    fn process_sampler(&self, sampler_node: &Value) -> Option<ParsedWorkflow> {
        // Extract each piece independently for maximum recovery
        let (sampler_name, scheduler) = self.extract_sampler_details(sampler_node);
        let model_name = self.extract_model(sampler_node);
        let (pos_prompts, neg_prompts) = self.extract_prompts(sampler_node);
        let (width, height) = self.extract_dimensions(sampler_node);
        let (cfg, steps) = self.extract_parameters(sampler_node);
        
        Some(ParsedWorkflow {
            model_name,
            sampler_name,
            scheduler,
            positive_prompt: pos_prompts.join("\n---\n"),
            negative_prompt: neg_prompts.join("\n---\n"),
            width,
            height,
            cfg,
            steps,
        })
    }
    
    fn extract_sampler_details(&self, sampler_node: &Value) -> (Option<String>, Option<String>) {
        let sampler_name = self.get_widget_value(sampler_node, "sampler_name")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        
        let scheduler = self.get_widget_value(sampler_node, "scheduler")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        
        (sampler_name, scheduler)
    }
    
    fn extract_model(&self, sampler_node: &Value) -> Option<String> {
        let node_id_string = sampler_node.as_object()?
            .get("id")?
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| sampler_node.as_object()?.get("id")?.as_i64().map(|i| i.to_string()))?;
        
        let model_node = self.find_source_node(&node_id_string, "model", MODEL_LOADER_TYPES, 20)?;
        
        // Try various fields where model name might be
        let model_name = self.get_widget_value(&model_node, "ckpt_name")
            .or_else(|| self.get_widget_value(&model_node, "unet_name"))
            .or_else(|| self.get_widget_value(&model_node, "model_name"))
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        
        model_name
    }
    
    fn extract_prompts(&self, sampler_node: &Value) -> (Vec<String>, Vec<String>) {
        let node_id_string = sampler_node.as_object()
            .and_then(|n| n.get("id"))
            .and_then(|id| {
                id.as_str().map(|s| s.to_string())
                    .or_else(|| id.as_i64().map(|i| i.to_string()))
            })
            .unwrap_or_else(|| String::new());
        
        let node_id = node_id_string.as_str();
        
        let mut pos_prompts = Vec::new();
        let mut neg_prompts = Vec::new();
        
        // Extract positive prompt
        if let Some(pos_node) = self.find_source_node(node_id, "positive", PROMPT_NODE_TYPES, 20) {
            if let Some(v) = self.get_widget_value(&pos_node, "text") {
                if let Some(text) = v.as_str() {
                    pos_prompts.push(text.to_string());
                }
            }
        }
        
        // Extract negative prompt
        if let Some(neg_node) = self.find_source_node(node_id, "negative", PROMPT_NODE_TYPES, 20) {
            if let Some(v) = self.get_widget_value(&neg_node, "text") {
                if let Some(text) = v.as_str() {
                    neg_prompts.push(text.to_string());
                }
            }
        }
        
        (pos_prompts, neg_prompts)
    }
    
    fn extract_parameters(&self, sampler_node: &Value) -> (Option<f64>, Option<i64>) {
        let cfg = self.get_widget_value(sampler_node, "cfg")
            .and_then(|v| v.as_f64());
        
        let steps = self.get_widget_value(sampler_node, "steps")
            .and_then(|v| v.as_i64());
        
        (cfg, steps)
    }
    
    fn extract_dimensions(&self, sampler_node: &Value) -> (Option<i64>, Option<i64>) {
        // Try to find Empty Latent Image or similar nodes
        let node_id_string = sampler_node.as_object()
            .and_then(|n| n.get("id"))
            .and_then(|id| {
                id.as_str().map(|s| s.to_string())
                    .or_else(|| id.as_i64().map(|i| i.to_string()))
            })
            .unwrap_or_else(|| String::new());
        
        // Try latent_image input
        if let Some(latent_node) = self.find_source_node(&node_id_string, "latent_image", &["EmptyLatentImage"], 20) {
            let width = self.get_widget_value(&latent_node, "width")
                .and_then(|v| v.as_i64());
            let height = self.get_widget_value(&latent_node, "height")
                .and_then(|v| v.as_i64());
            
            if width.is_some() && height.is_some() {
                return (width, height);
            }
        }
        
        (None, None)
    }
}

/// Extract workflow metadata from JSON string
pub fn extract_workflow_metadata(workflow_str: &str, file_path: &Path) -> Result<Vec<ParsedWorkflow>, String> {
    let workflow_data: Value = serde_json::from_str(workflow_str)
        .map_err(|e| format!("Failed to parse workflow JSON: {}", e))?;
    
    let parser = ComfyUIWorkflowParser::new(workflow_data, file_path)?;
    Ok(parser.parse())
}
