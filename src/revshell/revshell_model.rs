use std::fmt;

use json::JsonValue;

use crate::shared::xmind::XMindJson;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Revshell {
    title: String,
    description: String,
    class: String,
    labels: Vec<String>,
    href: String,
}

#[derive(Clone)]
pub struct RevShells {
    revshells: Vec<Revshell>,
}

impl RevShells {
    pub fn new(revshells: Vec<Revshell>) -> Self {
        Self { revshells }
    }

    pub fn revshells(&self) -> &Vec<Revshell> {
        &self.revshells
    }
}

impl fmt::Display for RevShells {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.revshells.iter().map(|revshell| revshell.title.to_string()).collect::<Vec<String>>().join(" | ")
        )
    }
}

#[allow(dead_code)]
impl Revshell {
    pub fn new(json_obj: &JsonValue) -> Self {
        Self {
            title: json_obj["title"].to_string(),
            description: json_obj["notes"]["plain"]["content"].to_string(),
            class: json_obj["class"].to_string(),
            labels: json_obj["labels"]
                .members()
                .map(|label| label.to_string())
                .collect(),
            href: json_obj["href"].to_string(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn file_path(&self) -> String {
        "/tmp/".to_string() + self.title()
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn command(&self, lhost: &str, lport: u16) -> String {
        self.description.replace("\n", "").replace("$LHOST", lhost).replace("$LPORT", lport.to_string().as_str()).trim().to_string()
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
    pub fn rev_type(&self) -> String {
        self.labels.first().unwrap().to_string()
    }
    pub fn link_name(&self) -> String {
        self.labels.first().unwrap().to_string() + "/" + self.title()
    }
}


impl XMindJson<RevShells> for RevShells {
    fn from_root_json(root_json: &JsonValue) -> RevShells {
        let mut all_revshels: Vec<JsonValue> = Vec::new();
        Self::recursive_search_with_label(
            &mut all_revshels,
            &root_json[0]["rootTopic"]["children"]["attached"].members().find(|member| member["title"].to_string() == "Reverse Shell").unwrap(),
            &vec!["msfvenom","xmind","local"],
            5,
        );

        RevShells::new(all_revshels.iter().map(|revshell| Revshell::new(revshell)).collect())
    }
}