use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// 常量 toml 文件路径
const TOML_PATH: &str = "/Users/ls/.config/dali/llm.toml";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LLMService {
    pub api_key: String,
    pub model_name: String,
    pub base_url: String,
    pub provider: String,
}

impl LLMService {
    pub fn new(api_key: &str, model_name: &str, base_url: &str, provider: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            model_name: model_name.to_string(),
            base_url: base_url.to_string(),
            provider: provider.to_string(),
        }
    }
}

// 定义一个结构来存储多个LLM服务
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LLMServiceList {
    pub services: Vec<LLMService>,
}

// LLMServiceList struct的实现方法
impl LLMServiceList {
    // 创建新 LLMServiceList 的构造函数方法
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    // 保存到 toml 文件
    pub fn save(&self) -> Result<(), std::io::Error> {
        let toml = toml::to_string(&self).unwrap();
        std::fs::write(TOML_PATH, toml)?;
        Ok(())
    }

    // 从 toml 文件加载
    pub fn load() -> Result<Self, std::io::Error> {
        let toml = std::fs::read_to_string(TOML_PATH)?;
        let services: LLMServiceList = toml::from_str(&toml).unwrap();
        Ok(services)
    }

    // 向注册表添加新 LLM 服务的方法
    pub fn add_service(&mut self, service: LLMService) {
        self.services.push(service);
    }

    // 查找服务
    pub fn find_service(&self, provider_name: String) -> Option<&LLMService> {
        let mut map = HashMap::new();
        map.insert("通义千问", "qwen");
        map.insert("月之暗面", "moonshot");
        map.insert("零一万物", "yi");
        map.insert("深度求索", "deepseek");
        map.insert("Groq", "groq");

        let provider_name = map.get(&provider_name.as_str()).unwrap();
        self.services
            .iter()
            .find(|service| &service.provider.as_str() == provider_name)
    }
}
