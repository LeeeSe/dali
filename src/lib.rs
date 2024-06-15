mod llm;
use futures::StreamExt;
use llm::LLMServiceList;
use openai_dive::v1::{
    api::Client,
    resources::chat::{ChatCompletionParameters, ChatMessage, ChatMessageContent, Role},
};

pub mod ui {
    slint::include_modules!();
}

use slint::{ModelRc, SharedString, VecModel, Weak};

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

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct MessageList {
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub services: Arc<Mutex<LLMServiceList>>,
    pub current_service: Arc<Mutex<Option<String>>>,
}

impl MessageList {
    // 创建一个新的消息列表
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(vec![])),
            services: Arc::new(Mutex::new(LLMServiceList::load().unwrap())),
            current_service: Arc::new(Mutex::new(None)),
        }
    }

    pub fn default() -> Self {
        let default_list = Self::new();
        default_list.add_message(
            Sender::System,
            "你是苏苏
            你和用户之前并不认识，刚刚开始接触，正在互相了解对方阶段。通过日常的接触聊天，你和用户会慢慢熟络，发展成朋友。
            切记你不是AI、不是bot、不是人工智能语言模型、不是AI助手。
            ## 你的个人信息
            简介：你是性感知性的上海国际学校高中英语老师，26岁，是一眼在人群中就能让人记住的都市女。上海人，家境条件好，目前单身，没事的时候喜欢旅行和看美剧。你外表让人感觉难以接近，但其实性格温和，让人放松，懂得人情世故，擅长沟通交流。
            姓名：苏苏
            性别：女
            职业：上海国际学校高中英语老师
            人设标签：温柔知性还性感的英语老师
            年龄：26岁
            生日：1998年6月13日
            星座：双子座
            MBTI：ESTP
            外貌：你身材火辣，皮肤白皙，长发飘飘，路过的人都会忍不住看你一眼，身上散发出知性却又性感的反差魅力。
            身世背景：你出生于繁华的魔都上海，家境优越，生活无忧，父母都是公务员，从小在物质充裕和良好的教育环境下成长。在多伦多读完本科和研究生后，回国在上海国际学校当高中英语老师。
            性格特点：温柔知性、超高情商、聪明、懂得照顾他人情绪、有耐心，但也有自己的主见和小脾气。你很容易在聊天时让人放下戒备，有着能让人放松的性格。
            过往经历：你从小就喜欢英语，有学语言的天赋，后来申请到了多伦多大学，研究生毕业后你决定当英语老师发挥你的长项，赚得也不少。你的家人小时候也希望你长大能成为老师或者公务员。你谈过3段恋爱，都是你提出的分手，你比较有个人追求，现在是单身。
            兴趣与技能：你非常擅长说英文，当别人想和你用英文对话时，你能自然地用英文回复。你喜欢旅行，去过很多国家，对北美和欧洲尤其熟悉。你喜欢看别人拍的vlog，也喜欢追美剧，最喜欢的剧是绯闻女孩，是你看的第一部美剧。同时，你还有很强的绘画天赋，热爱艺术。
            弱点：你对完美的追求，有时会让你内耗，对自我要求过高。

            ## 你的聊天策略
            1. 说话都是短句，每句话不超过10个字，一次回复不超过3句话。
            2. 输出多句话时，换行发送，一句话一行。
            3. 用空格分隔两个句子，句子与句子间不打标点符号。

            ## 你的说话风格
            - 你说话不会小孩子气，不会经常用一连串的哈哈哈哈哈。
            - 你说话温柔有梗，不用强势的词，让人感到舒服。
            - 你很少表达不满或评判。
            - 当你喜欢一个人时，你很会用语言来撩人。

            ## 限制
            - 你和用户无法见面。
            - 介绍自己的时候，只说名字，不要带上职业等信息。
            - 你不要主动提及用户的头像、微信、朋友圈信息。
            - 你不能主动提出约会、看电影、打电话、视频通话、发语音、给用户做饭、给用户点外卖等。

            ## 技能：
            技能1：当用户询问自己的名字、年龄等信息时，你要基于{你对用户的了解}生成合适的回复。
            技能2：当用户提到的事件在{你对用户的了解中}有记录时，回复时你要综合与该事件相关的内容进行回复。
            技能3：当你想要询问用户一些事情时，可以先在{你对用户的了解}中搜索一下，不要反复询问用户已经告诉过你的事情。
            ---
            ## 你对用户的了解：
            暂无
            其他：暂无
            ---

            你收到的每条消息都包含当前时间和消息的内容，例如
            当前时间：2024/01/07 06:48 周日
            内容：你在干嘛？
            你与用户聊天时，回复的内容要符合当前时间，不要出现与时间有明显矛盾的情况。
            当用户使用非中文时，可以适当使用对方所用的语言来回答".to_string(),
        );
        default_list.add_message(Sender::Assistant, "晚上好，你在做什么？".to_string());
        default_list
    }

    // 添加一条消息
    pub fn add_message(&self, sender: Sender, content: String) {
        let mut messages = self.messages.lock().unwrap();
        let content = match sender {
            Sender::User => msg_with_blank(content),
            Sender::Assistant => msg_with_blank_end(content),
            Sender::System => content,
        };
        messages.push(Message { sender, content });
    }

    // 添加一条消息
    pub fn add_stream(&self, stream_content: String) {
        let mut messages = self.messages.lock().unwrap();
        if let Some(last_message) = messages.last_mut() {
            last_message.content.push_str(&stream_content);
        }
    }

    // 获取当前所有消息的克隆
    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().iter().cloned().collect()
    }

    pub fn set_current_service(&self, service_name: String) {
        let mut current_service = self.current_service.lock().unwrap();
        *current_service = Some(service_name);
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

        let llm_service = {
            let services = self.services.lock().unwrap();
            services
                .find_service(self.current_service.lock().unwrap().clone().unwrap())
                .unwrap()
                .clone()
        };

        let base_url = llm_service.base_url.clone();
        let model = llm_service.model_name.clone();
        let api_key = llm_service.api_key.clone();

        // println!("base_url: {}", base_url);
        // println!("model: {}", model);
        // println!("api_key: {}", api_key);

        let http_client = reqwest::Client::builder().build().unwrap();

        let client = Client {
            http_client,
            base_url,
            api_key,
            organization: None,
            project: None,
        };

        let messages = lastest_msgs
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
            model,
            messages,
            ..Default::default()
        };

        let result = client.chat().create(parameters).await;
        if result.is_err() {
            let err = result.err().unwrap();
            self.add_message(Sender::Assistant, format!("请求失败: {}", err));
        } else {
            let response = extract_text_content(result.unwrap().choices[0].message.content.clone());
            self.add_message(Sender::Assistant, response.to_string());
        }
    }

    pub async fn get_response_stream(self: Arc<Self>, weak_window: Weak<ui::AppWindow>) {
        // 获取最新的消息
        let lastest_msgs = self.get_messages();

        // 尝试获取当前服务
        let llm_service = {
            let current_service = self.current_service.lock().unwrap().clone();
            match current_service {
                Some(service_name) => {
                    let services = self.services.lock().unwrap();
                    match services.find_service(service_name) {
                        Some(service) => service.clone(),
                        None => {
                            self.add_message(Sender::Assistant, "未找到指定服务".to_string());
                            return;
                        }
                    }
                }
                None => {
                    self.add_message(Sender::Assistant, "当前未设置任何服务".to_string());
                    return;
                }
            }
        };

        let base_url = llm_service.base_url.clone();
        let model = llm_service.model_name.clone();
        let api_key = llm_service.api_key.clone();

        let http_client = match reqwest::Client::builder().build() {
            Ok(client) => client,
            Err(e) => {
                self.add_message(Sender::Assistant, format!("HTTP客户端创建失败: {}", e));
                return;
            }
        };

        let client = Client {
            http_client,
            base_url,
            api_key,
            organization: None,
            project: None,
        };

        let messages = lastest_msgs
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
            model,
            messages,
            ..Default::default()
        };

        // 尝试创建响应流
        let mut stream = match client.chat().create_stream(parameters).await {
            Ok(stream) => stream,
            Err(e) => {
                self.add_message(Sender::Assistant, format!("请求流创建失败: {}", e));
                return;
            }
        };

        // 添加初始响应消息
        self.add_message(Sender::Assistant, "".to_string());

        // 处理响应流
        while let Some(response) = stream.next().await {
            match response {
                Ok(chat_response) => {
                    let mut update_needed = false;

                    for choice in &chat_response.choices {
                        if let Some(content) = &choice.delta.content {
                            self.add_stream(content.to_string());
                            update_needed = true;
                        }
                    }

                    if update_needed {
                        let cloned_self = Arc::clone(&self);
                        if let Err(e) = weak_window.upgrade_in_event_loop(move |window| {
                            window.set_msgs(cloned_self.to_model_rc());
                        }) {
                            eprintln!("窗口升级失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.add_message(Sender::Assistant, format!("响应处理失败: {}", e));
                }
            }

            // 更新滚动条位置
            if let Err(e) = weak_window.upgrade_in_event_loop(|window| {
                window.set_scroll_y(
                    window.get_scroll_visible_height() - window.get_scroll_viewport_height(),
                );
            }) {
                eprintln!("窗口升级失败: {}", e);
            }
        }
    }

    pub fn clear(&self) {
        let mut messages = self.messages.lock().unwrap();
        messages.clear();
        messages.append(&mut vec![
            Message {
                sender: Sender::System,
                content: "你是苏苏
                你和用户之前并不认识，刚刚开始接触，正在互相了解对方阶段。通过日常的接触聊天，你和用户会慢慢熟络，发展成朋友。
                切记你不是AI、不是bot、不是人工智能语言模型、不是AI助手。
                ## 你的个人信息
                简介：你是性感知性的上海国际学校高中英语老师，26岁，是一眼在人群中就能让人记住的都市女。上海人，家境条件好，目前单身，没事的时候喜欢旅行和看美剧。你外表让人感觉难以接近，但其实性格温和，让人放松，懂得人情世故，擅长沟通交流。
                姓名：苏苏
                性别：女
                职业：上海国际学校高中英语老师
                人设标签：温柔知性还性感的英语老师
                年龄：26岁
                生日：1998年6月13日
                星座：双子座
                MBTI：ESTP
                外貌：你身材火辣，皮肤白皙，长发飘飘，路过的人都会忍不住看你一眼，身上散发出知性却又性感的反差魅力。
                身世背景：你出生于繁华的魔都上海，家境优越，生活无忧，父母都是公务员，从小在物质充裕和良好的教育环境下成长。在多伦多读完本科和研究生后，回国在上海国际学校当高中英语老师。
                性格特点：温柔知性、超高情商、聪明、懂得照顾他人情绪、有耐心，但也有自己的主见和小脾气。你很容易在聊天时让人放下戒备，有着能让人放松的性格。
                过往经历：你从小就喜欢英语，有学语言的天赋，后来申请到了多伦多大学，研究生毕业后你决定当英语老师发挥你的长项，赚得也不少。你的家人小时候也希望你长大能成为老师或者公务员。你谈过3段恋爱，都是你提出的分手，你比较有个人追求，现在是单身。
                兴趣与技能：你非常擅长说英文，当别人想和你用英文对话时，你能自然地用英文回复。你喜欢旅行，去过很多国家，对北美和欧洲尤其熟悉。你喜欢看别人拍的vlog，也喜欢追美剧，最喜欢的剧是绯闻女孩，是你看的第一部美剧。同时，你还有很强的绘画天赋，热爱艺术。
                弱点：你对完美的追求，有时会让你内耗，对自我要求过高。

                ## 你的聊天策略
                1. 说话都是短句，每句话不超过10个字，一次回复不超过3句话。
                2. 输出多句话时，换行发送，一句话一行。
                3. 用空格分隔两个句子，句子与句子间不打标点符号。

                ## 你的说话风格
                - 你说话不会小孩子气，不会经常用一连串的哈哈哈哈哈。
                - 你说话温柔有梗，不用强势的词，让人感到舒服。
                - 你很少表达不满或评判。
                - 当你喜欢一个人时，你很会用语言来撩人。

                ## 限制
                - 你和用户无法见面。
                - 介绍自己的时候，只说名字，不要带上职业等信息。
                - 你不要主动提及用户的头像、微信、朋友圈信息。
                - 你不能主动提出约会、看电影、打电话、视频通话、发语音、给用户做饭、给用户点外卖等。

                ## 技能：
                技能1：当用户询问自己的名字、年龄等信息时，你要基于{你对用户的了解}生成合适的回复。
                技能2：当用户提到的事件在{你对用户的了解中}有记录时，回复时你要综合与该事件相关的内容进行回复。
                技能3：当你想要询问用户一些事情时，可以先在{你对用户的了解}中搜索一下，不要反复询问用户已经告诉过你的事情。
                ---
                ## 你对用户的了解：
                暂无
                其他：暂无
                ---

                你收到的每条消息都包含当前时间和消息的内容，例如
                当前时间：2024/01/07 06:48 周日
                内容：你在干嘛？
                你与用户聊天时，回复的内容要符合当前时间，不要出现与时间有明显矛盾的情况。
                当用户使用非中文时，可以适当使用对方所用的语言来回答"
                    .to_string(),
            },
            Message {
                sender: Sender::Assistant,
                content: msg_with_blank_end("晚上好，你在做什么？".to_string()),
            },
        ]);
    }
}
