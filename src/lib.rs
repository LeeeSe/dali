use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent, Role},
};
use slint::{ModelRc, SharedString, VecModel};

pub fn msg_with_blank_end(mut msg: String) -> String {
    msg.push(' '); // 尾部添加空格
    msg
}

pub fn msg_with_blank(msg: String) -> String {
    let mut new_string = String::from(" "); // 头部添加空格
    new_string.push_str(msg.as_str()); // 添加原始字符串
    new_string.push(' '); // 尾部添加空格
    new_string
}

pub async fn get_response(msgs: Vec<Message>) -> SharedString {
    let api_key = "sk-c6b066326ec04fc1a726cb9293b8f02b".to_string();
    let base_url = "https://api.deepseek.com".to_string();

    let http_client = reqwest::Client::builder().build().unwrap();

    let client = Client {
        http_client,
        base_url,
        api_key,
        organization: None,
        project: None,
    };

    let messages = msgs
        .iter()
        .map(|msg| ChatMessage {
            role: match msg.sender {
                Sender::User => Role::User,
                Sender::System => Role::System,
                Sender::Assistant => Role::Assistant,
            },
            content: ChatMessageContent::Text(msg.content.clone()),
            ..Default::default()
        })
        .collect();

    let parameters = ChatCompletionParameters {
        model: "deepseek-chat".to_string(),
        messages,
        max_tokens: Some(4096),
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    let response = extract_text_content(result.choices[0].message.content.clone());

    SharedString::from(response)
}

fn extract_text_content(content: ChatMessageContent) -> String {
    match content {
        ChatMessageContent::Text(text) => text,
        _ => "未知返回".to_string(),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sender {
    User,
    System,
    Assistant,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub sender: Sender,
    pub content: String,
}

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct MessageList {
    pub messages: Arc<Mutex<Vec<Message>>>,
}

impl MessageList {
    // 创建一个新的消息列表
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn default() -> Self {
        let default_list = Self::new();
        default_list.add_message(
            Sender::System,
            "你是一个具有各种行业专业知识的机器人助手，你使用中文来回答问题".to_string(),
        );
        default_list.add_message(
            Sender::Assistant,
            "您好，请问有什么可以帮您的吗？".to_string(),
        );
        default_list
    }

    // 添加一条消息
    pub fn add_message(&self, sender: Sender, content: String) {
        let mut messages = self.messages.lock().unwrap();
        match sender {
            Sender::User => {
                let content = msg_with_blank(content);
                messages.push(Message { sender, content });
            }
            Sender::System => {
                messages.push(Message { sender, content });
            }
            Sender::Assistant => {
                let content = msg_with_blank_end(content);
                messages.push(Message { sender, content });
            }
        }
    }

    // 获取当前所有消息的克隆
    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }

    // 生成可供 ui.set_msgs 输入的内容
    pub fn to_model_rc(&self) -> ModelRc<(i32, SharedString)> {
        let messages = self.get_messages();
        let model = VecModel::from(
            messages
                .iter()
                .enumerate()
                .map(|(_index, message)| {
                    (
                        match message.sender {
                            Sender::System => 0,
                            Sender::User => 1,
                            Sender::Assistant => 2,
                        },
                        SharedString::from(message.content.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        );

        ModelRc::new(model)
    }

    // 异步添加响应消息
    pub async fn get_response(&self) {
        let lastest_msgs = self.get_messages();
        let response = get_response(lastest_msgs).await;
        self.add_message(Sender::Assistant, response.to_string());
    }

    pub fn clear(&self) {
        let mut messages = self.messages.lock().unwrap();
        messages.clear();
        messages.append(&mut vec![
            Message {
                sender: Sender::System,
                content: "你是一个具有各种行业专业知识的机器人助手，你使用中文来回答问题"
                    .to_string(),
            },
            Message {
                sender: Sender::Assistant,
                content: msg_with_blank_end("您好，请问有什么可以帮您的吗？".to_string()),
            },
        ]);
    }
}
