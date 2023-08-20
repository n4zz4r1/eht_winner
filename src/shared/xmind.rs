/* XMIND Utils module from Shared
  [X] done
  [X] refactor
*/
use json::JsonValue;
use std::io::Read;

pub trait XMindJson<T> {
    fn from_root_json(root_json: &JsonValue) -> T;
    fn recursive_search_with_label(
        objs_with_label: &mut Vec<JsonValue>,
        current_obj: &JsonValue,
        labels: &Vec<&str>,
        deep: i32,
    ) {
        // check if has label
        if !current_obj["labels"].is_empty()
            && current_obj["labels"]
                .members()
                .any(|label| labels.contains(&label.as_str().unwrap()))
        {
            objs_with_label.push(current_obj.clone());
        }

        // avoid infinite loop !!
        if deep <= 1 {
            return;
        }

        // search on childs
        for child in &current_obj["children"]["attached"]
            .members()
            .collect::<Vec<_>>()
        {
            Self::recursive_search_with_label(objs_with_label, child, labels, deep - 1);
        }
    }
}

pub fn get_content_from_xmind() -> JsonValue {
    let path: &str = "/opt/winner/winner.xmind";
    let file: std::fs::File =
        std::fs::File::open(path).expect("/opt/winner/winner.xmind not found");
    let mut zip_file = zip::ZipArchive::new(file).expect("winner.xmind is not a zip file");
    let mut content_file = zip_file
        .by_name("content.json")
        .expect("winner.json inside winner.xmind not found.");
    let mut contents = String::new();
    content_file.read_to_string(&mut contents).unwrap();

    json::parse(contents.as_str()).unwrap()
}
