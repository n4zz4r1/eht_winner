/* Utils module from Shared
  [ ] done
  [X] refactor
*/
use std::collections::HashMap;
use std::fmt;

use json::JsonValue;

use crate::logger_info;
use crate::shared::xmind::XMindJson;
use crate::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Tool {
    title: String,
    description: String,
    class: String,
    labels: Vec<String>,
    href: String,
    os: String,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "'{}', '{}', '{}', '{}', '{}', '{}'",
            self.title,
            self.description,
            self.class,
            self.labels.join(" - "),
            self.href,
            self.os
        )
    }
}

#[derive(Clone)]
pub struct Tools {
    tools_by_os: HashMap<String, Vec<Tool>>,
}

#[allow(dead_code)]
impl Tool {
    pub fn new(json_obj: &JsonValue, os: &str) -> Self {
        Self {
            title: json_obj["title"].to_string(),
            description: json_obj["notes"]["plain"]["content"].to_string(),
            class: json_obj["class"].to_string(),
            labels: json_obj["labels"]
                .members()
                .map(|label| label.to_string())
                .collect(),
            href: json_obj["href"].to_string(),
            os: os.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn class(&self) -> &str {
        &self.class
    }
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }
    pub fn href(&self) -> &str {
        &self.href
    }
    pub fn os(&self) -> &str {
        &self.os
    }
}

impl XMindJson<Tools> for Tools {
    fn from_root_json(root_json: &JsonValue) -> Tools {
        let mut tools_by_os: HashMap<String, Vec<Tool>> = HashMap::new();
        let mut objects_with_os: Vec<JsonValue> = Vec::new();
        Self::recursive_search_with_label(
            &mut objects_with_os,
            &root_json[0]["rootTopic"],
            &vec!["os"],
            3,
        );

        for object_with_os in &objects_with_os {
            let mut tools_from_os: Vec<JsonValue> = Vec::new();
            Self::recursive_search_with_label(
                &mut tools_from_os,
                object_with_os,
                &vec!["url", "local"],
                20,
            );
            let tools: Vec<String> = tools_from_os
                .iter()
                .map(|json| json["title"].to_string())
                .collect();
            logger_info!(format!(
                "OS {} was found with {} tool(s) : {}",
                object_with_os["title"].to_string().green().bold(),
                tools_from_os.len().to_string().green(),
                tools.join(" | ").green()
            ));
            tools_by_os.insert(
                object_with_os["title"].to_string(),
                tools_from_os
                    .iter()
                    .map(|json| Tool::new(json, &object_with_os["title"].to_string()))
                    .collect(),
            );
        }
        Tools::new(tools_by_os)
    }
}

impl Tools {
    pub fn new(tools_by_os: HashMap<String, Vec<Tool>>) -> Self {
        Self { tools_by_os }
    }
    #[allow(dead_code)]
    pub fn print_all(&self) {
        let _ = &self.tools_by_os.iter().for_each(|os| {
            os.1.iter().for_each(|tool| {
                logger_info!(format!("found {}", tool.to_string()));
            })
        });
    }
    pub fn exists(&self, os: &str, tool_name: &str) -> bool {
        self.tools_by_os.contains_key(os)
            && self
                .tools_by_os
                .get(os)
                .unwrap()
                .iter()
                .any(|tool| tool.title == tool_name)
    }
    pub fn get_by_os_and_name(&self, os: &str, tool_name: &str) -> &Tool {
        self.tools_by_os
            .get(os)
            .unwrap()
            .iter()
            .find(|tool| tool.title == tool_name)
            .unwrap()
    }
}
