use std::io::Read;
use std::io::Write;
use std::os::windows::process::CommandExt;
use dioxus::prelude::*;
use super::app::*;

#[derive(Debug, Clone)]
pub struct ResultSearch {
    pub search_message: String,
    pub is_search: bool,
    pub search_memos: Vec<SearchMemo>,
}
impl ResultSearch{
    pub fn new(search_message: &str, is_search: bool, search_memos: Vec<SearchMemo>) -> ResultSearch{
        ResultSearch { 
            search_message: search_message.into(),
            is_search: is_search,
            search_memos: search_memos 
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchMemo {
    pub path: String,
    pub name: String,
    pub contents: Vec<String>,
}
impl SearchMemo{
    pub fn new(path: &str, search: &str) -> SearchMemo{
        let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap().to_string();
        let mut f = std::fs::File::open(path).unwrap();
        let search_string = String::from(search);
        let mut content = String::new();
        f.read_to_string(&mut content).expect("something went wrong reading the file");
        let splits = split_keep(&content, &search_string);
        let mut contents = Vec::new();
        for s in splits{
            contents.push(String::from(s));
        }
        SearchMemo { path: path.replace("\\","/"), name: name, contents: contents}
    }
    
    pub fn contains(&self, hit: &str) -> bool{
        for t in &self.contents{
            if t.contains(hit){return true;}
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct Memo {
    pub path: String,
    pub name: String,
    pub content: String,
}
impl Memo{
    pub fn new(path: &str) -> Memo{
        let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap().to_string();
        let mut f = std::fs::File::open(path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).expect("something went wrong reading the file");
        Memo { path: path.to_string(), name: name, content: content }
    }
    
    pub fn write_memo(&mut self, value: &str){
        self.content = value.to_string();
        let mut file = std::fs::File::create(&self.path).expect("create failed");
        file.write_all(self.content.as_bytes()).expect("write failed");
    }
}

#[derive(Debug, Clone)]
pub struct MemoMenu{
    pub is_show_menu: bool,
    pub memo_path: String,
    pub pos_x: f64,
    pub pos_y: f64,
}

impl Default for MemoMenu{
    fn default() -> MemoMenu{
        MemoMenu { 
            is_show_menu: false, 
            memo_path: String::new(),
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }
}

impl MemoMenu{
    pub fn open_memo(&self){
        let _res = std::process::Command::new("cmd.exe")
            .args(["/c", "start", &self.memo_path])
            .creation_flags(0x08000000)
            .spawn().unwrap();
    }
    pub fn open_folder(&self){
        std::process::Command::new("explorer.exe").arg(format!("{}{}", "/select,",self.memo_path.replace("/","\\"))).status().unwrap();
    }
}

pub async fn create_memo(mut app: Signal<App>){
    if app().tab_set.tabs.len() == 0 {
        let _ = native_dialog::MessageDialog::new()
            .set_type(native_dialog::MessageType::Info)
            .set_title("確認")
            .set_text("タブを作成してください")
            .show_alert();
        return;
    }else if  app().tab_set.tabs.len() < app().json.current_num{
        let _ = native_dialog::MessageDialog::new()
            .set_type(native_dialog::MessageType::Info)
            .set_title("確認")
            .set_text("タブを選択してください")
            .show_alert();
        return;
    }
    let datetime = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()).naive_local();
    let tab_name = app().tab_set.tabs[app().json.current_num].name.to_string();
    let date_txt = format!("{}_{}{}",datetime.format("%Y%m%d%H%M%S").to_string(),&tab_name,".txt");
    let n_memo_path = format!("{}/{}/{}", get_exe_path(), &app().tab_set.tabs[app().json.current_num].name, date_txt);
    let _ = std::fs::File::create(&n_memo_path);
    app.write().is_reset_memo = true;
}

pub async fn delete_memo(mut app: Signal<App>){
    let res = native_dialog::MessageDialog::new()
        .set_type(native_dialog::MessageType::Info)
        .set_title("確認")
        .set_text("削除しますか?")
        .show_confirm();
    if res.is_err(){return;}
    if !res.unwrap(){return;}
    
    let _ = std::fs::remove_file(&app().menu.memo_path);
    app.write().is_reset_memo = true;
}

fn split_keep(text: &str, search: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(search) {
        if last != index {
            result.push(String::from(&text[last..index]));
        }
        result.push(String::from(matched));
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(String::from(&text[last..]));
    }
    result
}