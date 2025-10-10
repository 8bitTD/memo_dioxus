use dioxus::prelude::*;
use dioxus::desktop::*;
use dioxus::desktop::tao::event::Event;
use tao::event;

use super::json::*;
use super::define::*;
use super::memo;

#[derive(Debug, Clone, PartialEq)]
pub enum State{
    Home,
    EditTab,
    AddTab,
    Search,
}

#[derive(Debug, Clone)]
pub struct Tab{
    pub name: String,
}
impl Default for Tab{
    fn default() -> Tab{
        Tab { 
            name: String::from(""),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TabSet{
    pub tabs: Vec<Tab>,
    pub memos: Vec<memo::Memo>,
    pub current_memo_path: String,
}
impl Default for TabSet{
    fn default() -> TabSet{
        TabSet { tabs: Vec::new(), memos: Vec::new(), current_memo_path: String::new() }
    }
}
impl TabSet{
    pub fn new(tabs: Vec<Tab>, memos: Vec<memo::Memo>, current_memo_path: String) -> TabSet{
        TabSet { tabs: tabs, memos: memos, current_memo_path: current_memo_path }
    }

    pub fn clear(&mut self){
        self.memos.clear();
        self.tabs.clear();
        self.current_memo_path = String::new();
    }
}

#[derive(Debug, Clone)]
pub struct App{
    pub json: Json,
    pub tab_set: TabSet,
    pub menu: memo::MemoMenu,
    pub is_reset_memo: bool,
    pub is_request_focus: bool,
    pub state: State,
    pub edit_tab: Tab,
    pub is_show_dialog: bool,
    pub is_pressed_ctrl: bool,
    pub search_string: String,
    pub is_search: bool,
    pub search_memos: Vec<memo::SearchMemo>,
    pub search_message: String,
}
impl Default for App{
    fn default() -> App{
        App { 
            json: Json::new(), 
            tab_set: TabSet::default(),
            menu: memo::MemoMenu::default(), 
            is_reset_memo: true,
            is_request_focus: false,
            state: State::Home,
            edit_tab: Tab::default(),
            is_show_dialog: false,
            is_pressed_ctrl: false,
            search_string: String::new(),
            is_search: false,
            search_memos: Vec::new(),
            search_message: String::new(),
        }
    }
}

pub fn get_exe_path() -> String{
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    let exe_path = path.to_str().unwrap().replace("\\","/");
    return exe_path;
}

pub fn use_effect_memo_reset(mut app: Signal<App>, mut elements: Signal<Vec::<std::rc::Rc<MountedData>>>, rx_reset_memo: Coroutine<TabSet>){
    if app().is_reset_memo{
        let tx = rx_reset_memo.tx();
        app.write().tab_set.clear();
        elements.write().clear();
        let current_num = app().json.current_num;
        std::thread::spawn(move || { reset_memo(current_num, tx); });
        app.write().is_reset_memo = false;
        app.write().is_request_focus = true;
    }  
}

pub fn use_effect_focus(mut app: Signal<App>, elements: Signal<Vec::<std::rc::Rc<MountedData>>>){
    if app().is_request_focus && elements.len() != 0 && elements.len() == app().tab_set.memos.len(){
        if let Some(element) = elements.with(|f| f.get(0).cloned()) {
            _ = element.set_focus(true);                
        }
        app.write().is_request_focus = false;
    }
}

pub fn event_handler<UserWindowEvent>(event: &Event<UserWindowEvent>, mut app: Signal<App>){
    if let Event::WindowEvent{//ウィンドウサイズ変更時の処理
        event: WindowEvent::Resized(size),
        ..
    } = event {
        app.write().json.wi.width = size.width;
        app.write().json.wi.height = size.height;
    }
    if let Event::WindowEvent{//ウィンドウ位置変更時の処理
        event: WindowEvent::Moved(pos),
        ..
    } = event {
        app.write().json.wi.pos_x = pos.x;
        app.write().json.wi.pos_y = pos.y;
    }
    if let Event::WindowEvent{//exe終了時に情報保存する処理
        event: WindowEvent::CloseRequested, 
        ..
    } = event {
        app().json.save();
        //if &get_exe_path() == common::EXEPATH {kill_dx();}
    }  

    if let Event::WindowEvent {//フォーカスが変更されたときは、ctrlを押した判定をfalseにする
        event: WindowEvent::Focused(_new_focused),
        ..
    } = event
    {
        app.write().is_pressed_ctrl = false;
    }

    if let Event::DeviceEvent{
        event: dioxus::desktop::tao::event::DeviceEvent::Key(key),
        ..
    } = event {
        if app().state == State::Home{
            if key.state == event::ElementState::Released && key.physical_key == tao::keyboard::KeyCode::F5 {//F5を押した時にメモを更新する
                app.write().is_reset_memo = true;
            }
            if key.state == event::ElementState::Pressed && key.physical_key == tao::keyboard::KeyCode::ControlLeft{
                app.write().is_pressed_ctrl = true;
            }
            if key.state == event::ElementState::Released && key.physical_key == tao::keyboard::KeyCode::ControlLeft{
                app.write().is_pressed_ctrl = false;
            }
            if key.state == event::ElementState::Pressed && key.physical_key == tao::keyboard::KeyCode::KeyN && app().is_pressed_ctrl{
                spawn(memo::create_memo(app));
            }
            if key.state == event::ElementState::Pressed && key.physical_key == tao::keyboard::KeyCode::KeyT && app().is_pressed_ctrl{
                app.write().state = State::AddTab;
            }
        }
    }
}

pub fn use_effect_get_search_memos(app: Signal<App>, rx_search_memo: Coroutine<memo::ResultSearch>){
    if app().is_search{
        let tx = rx_search_memo.tx();
        let mut memos = Vec::new();
        let mut tab_paths = Vec::new();
        let exe_path = get_exe_path();
        let msg = format!("ファイルを取得中です...");
        let rs = memo::ResultSearch::new(&msg, true, memos.clone());
        let _ = tx.unbounded_send(rs);
        let path = std::fs::read_dir(&exe_path).unwrap();
        for p in path {
            let tmp_p = p.unwrap().path();
            if !tmp_p.is_dir() {continue;}
            let folder = tmp_p.file_name().unwrap().to_str().unwrap().to_string();
            if folder.contains("memo_dioxus.exe") || &folder == "assets"{continue;}
            let tmp_path = format!("{}/{}", &exe_path, folder);
            tab_paths.push(tmp_path);
        }
        for tp in tab_paths{
            let paths = std::fs::read_dir(&tp).unwrap();
            for path in paths {
                let tmp_p = path.unwrap().path();
                if tmp_p.extension() != Some(std::ffi::OsStr::new("txt")){continue;}
                let p = tmp_p.to_str().unwrap();
                let m = memo::SearchMemo::new(p, &app().search_string.to_string());
                if !m.contains(&app().search_string){continue;}
                memos.push(m);
                let msg = format!("{} ファイルがヒットしました。", memos.len());
                let rs = memo::ResultSearch::new(&msg, true, memos.clone());
                let _ = tx.unbounded_send(rs);
            }
            memos.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        }
        let num = memos.len();
        let msg = format!("{} ファイルがヒットしました。", num);
        let rs = memo::ResultSearch::new(&msg, false, memos.clone());
        let _ = tx.unbounded_send(rs);
    }
}

fn reset_memo(current_num: usize, tx: UnboundedSender<TabSet>){
    let mut tabs = Vec::new();
    let mut current_memo_path = String::new();
    let mut memos = Vec::new();
    let paths = std::fs::read_dir(&get_exe_path()).unwrap();
    for path in paths {
        let tmp_p = path.unwrap().path();
        if !tmp_p.is_dir() {continue;}
        let tab = tmp_p.file_name().unwrap().to_str().unwrap().to_string();
        if tab.contains("memo_dioxus.exe") || &tab == "assets"{continue;}
        tabs.push(Tab{name: tab});
    }
    if tabs.len() > 0 && tabs.len() > current_num{
        current_memo_path = format!("{}/{}",get_exe_path(),&tabs[current_num].name);
        if std::path::Path::new(&current_memo_path).is_dir(){
            let paths = std::fs::read_dir(&current_memo_path).unwrap();
            for path in paths {
                let tmp_p = path.unwrap().path();
                if tmp_p.extension() != Some(std::ffi::OsStr::new("txt")){continue;}
                let p = tmp_p.to_str().unwrap();
                let m = memo::Memo::new(p);
                memos.push(m);
            }
            memos.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        }  
    }
    let tab_set = TabSet::new(tabs, memos, current_memo_path);
    let _res = tx.unbounded_send(tab_set);
}