use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

    // 创建一个默认的 LLMServiceList
    pub fn default() -> Self {
        let mut services = Self::new();
        services.add_service(LLMService::new(
            "sk-42ea2ce6b4eb4018b48dc3c1469632a1",
            "deepseek-chat",
            "https://api.deepseek.com",
            "deepseek",
        ));

        services.add_service(LLMService::new(
            "ebf76549bd4a4627a4979768c7c9c11b",
            "yi-large-rag",
            "https://api.lingyiwanwu.com/v1",
            "yi",
        ));

        services.add_service(LLMService::new(
            "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",
            "llama3-70b-8192",
            "https://api.groq.com/openai/v1",
            "groq",
        ));

        services.add_service(LLMService::new(
            "sk-h0DITLfsI4Cko8NNyjVV2u273ma8Ppvd6mriTQnYkL5vFmZL",
            "moonshot-v1-32k",
            "https://api.moonshot.cn/v1",
            "moonshot",
        ));

        services.add_service(LLMService::new(
            "sk-ce3f98adce0856307a9f19d7efc1bc08",
            "Baichuan4",
            "https://api.baichuan-ai.com/v1",
            "baichuan",
        ));

        services.add_service(LLMService::new(
            "90BAcWRjfaGNrapG0JzAYVOBrIDcAuiU0uF74CrOiNHnLANy",
            "accounts/fireworks/models/qwen2-72b-instruct",
            "https://api.fireworks.ai/inference/v1",
            "qwen",
        ));

        services
    }

    // 保存到 toml 文件
    pub fn save(&self) -> Result<(), std::io::Error> {
        let toml_path = env_home::env_home_dir()
            .unwrap()
            .join(".config/dali/llm.toml");
        let toml = toml::to_string(&self).unwrap();
        std::fs::write(toml_path, toml)?;
        Ok(())
    }

    // 从 toml 文件加载
    pub fn load() -> Result<Self, std::io::Error> {
        let toml_path = env_home::env_home_dir()
            .unwrap()
            .join(".config/dali/llm.toml");
        // 如果文件不存在，返回一个默认的 LLMServiceList
        if !toml_path.exists() {
            return Ok(Self::default());
        }
        let toml = std::fs::read_to_string(toml_path)?;
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
        map.insert("百川智能", "baichuan");
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
