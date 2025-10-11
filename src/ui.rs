use dioxus::prelude::*;

use dioxus::desktop::use_wry_event_handler;
use futures_util::StreamExt;

use super::app;
use super::memo;

pub fn ui() -> Element {
    let mut app = use_signal(|| app::App::default());
    let tab = use_signal(|| app::Tab::default());
    let elements = use_signal(Vec::<std::rc::Rc<MountedData>>::new);

    let rx_reset_memo = use_coroutine(move |mut rx: UnboundedReceiver<app::TabSet>| async move {
        while let Some(res) = rx.next().await { 
            app.write().tab_set = res;
            if app().tab_set.memos.len() == 0{  app.write().is_request_focus = false; }
        }
    });
        
    use_effect(move || { app::use_effect_memo_reset(app, elements, rx_reset_memo); });//メモリセット
    use_effect(move || { app::use_effect_focus(app, elements); });//フォーカス
    use_wry_event_handler(move |event, _| { app::event_handler(event, app); });//ウィンドウ変更時の記録用処理
    
    let rx_search_memo = use_coroutine(move |mut rx: UnboundedReceiver<memo::ResultSearch>| async move {
        while let Some(res) = rx.next().await { 
            app.write().is_search = res.is_search;
            app.write().search_memos = res.search_memos;
            app.write().search_result_message = res.search_result_message;
        }
    });
    
    use_effect(move || { app::use_effect_get_search_memos(app, rx_search_memo); });//検索用のメモ取得処理

    rsx!{
        match app().state{
            app::State::Home =>  home(app, elements),
            app::State::EditTab => edit_tab(app),
            app::State::AddTab => add_tab(app, tab),
            app::State::Search => search(app),
        }
    }
}

pub fn search(mut app: Signal<app::App>) -> Element{
    rsx!{
        div {  
            style: "text-align: center;",
            form {  
                onsubmit: move|_|{  },
                input{
                    style: "width: 70vh;",
                    r#type: "text",
                    placeholder: "検索文字",
                    disabled: app().is_search,
                    onmounted: move|cx|{let _ = cx.data().set_focus(true);  },
                    oninput: move |e| app.write().search_string = e.value(),
                    value: "{app().search_string}"
                }
                button {  
                    disabled: app().is_search,
                    onclick: move |_| { 
                        if !app().search_string.is_empty(){
                            app.write().search_memos.clear();
                            app.write().search_result_message = String::new();
                            app.write().is_search = true;
                        }
                    },
                    "検索"
                }
                button {  
                    disabled: app().is_search,
                    onclick: move |_| {app.write().state = app::State::Home;},
                    "戻る"
                }
            }
        }
        div { 
            style:"position: relative; top: 5px; align-items: center; text-align: center;",
            if app().is_search{
                img{ 
                    style: "position:relative; left: 0px; top: 5px; margin-right: 5px;",
                    class: "rotating",
                    src: "https://img.icons8.com/external-others-inmotus-design/32/external-Loading-loaders-others-inmotus-design-18.png"
                }
            }
            label { "{app().search_result_message}" }
            
        }

        div { 
            style: "width: 100%; position:fixed; top: 68px; left:0px; text-align: center; height:calc(100% - 75px); overflow-y: auto; ",
            for m in app().search_memos{
                div {
                    style: "width:calc(100%-20px); margin-left: 10px; margin-right: 20px; text-align: left;",
                    title: m.path.to_string(),
                    hr{}
                    pre {
                        style: "width:100%; margin: 5px; white-space: pre-wrap;",
                        ondoubleclick: move|_|{
                            std::process::Command::new("explorer.exe").arg(format!("{}{}", "/select,", m.path.replace("/","\\"))).status().unwrap();
                        },
                        for t in &m.contents{
                            if t == &app().search_string{
                                b {  
                                    span {  
                                        style: "margin: 0px; padding: 0px;",
                                        "{t}"
                                    }
                                }
                            }else{
                                label {
                                    style: "margin: 0px; padding: 0px; font-weight: normal;",
                                    "{t}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn home(mut app: Signal<app::App>,  mut elements: Signal<Vec<std::rc::Rc<MountedData>>>) -> Element {    
    let is_busy = use_memo(move || { app().is_reset_memo || app().is_request_focus});
    rsx! {
        div {
            style: "margin: 0px; overflow-x: hidden; overflow-y: hidden; background-color: transparent;",
            onclick: move |_| { app.write().menu.is_show_menu = false; },
            div { 
                style: "display: flex; float: left; overflow-x: auto; overflow-y: hidden; width: calc(100% - 140px); height: 40px;",
                for (u, t) in app().tab_set.tabs.iter().enumerate(){
                    button {
                        style: if u == app().json.current_num {"height: 25px; font-weight:bold;"}else{"height: 25px; opacity: 0.5;"},
                        onclick: move |_| { 
                            app.write().json.current_num = u;
                            app.write().is_reset_memo = true;
                        }, 
                        "{t.name}",
                    }    
                }
            }
            div { 
                style: "float: right; height: 40px; top: 25px;",
                button { 
                    style: "position: relative; width: 22px; height: 22px; top: 1px; margin-right: 5px;",
                    title: "メモを作成",
                    disabled: "{is_busy.read()}",
                    onclick: move |_| async move { spawn(memo::create_memo(app)); },
                    img {  
                        style:"position:relative; left: -3px; top: -1px;",
                        src: "https://img.icons8.com/pieces/16/pencil-tip.png",
                    }
                }
                button { 
                    style: "position: relative; width: 22px; height: 22px; top: 3px; margin-right: 5px;",
                    title: "タブを編集",
                    disabled: "{is_busy.read()}",
                    onclick: move |_| {
                        if app().tab_set.tabs.len() > 0{
                            app.write().edit_tab = app().tab_set.tabs[app().json.current_num].clone();
                            app.write().state = app::State::EditTab;
                        }
                    },
                    img {  
                        style:"position:relative; left: -5px; top: -2px;",
                        src: "https://img.icons8.com/cotton/18/settings--v1.png",
                    }
                }
                button { 
                    style: "position: relative; width: 22px; height: 22px; top: 1px; margin-right: 5px;",
                    title: "タブを追加",
                    disabled: "{is_busy.read()}",
                    onclick: move |_| { app.write().state = app::State::AddTab }, 
                    img {  
                        style:"position:relative; left: -3px; top: -1px;",
                        src: "https://img.icons8.com/arcade/16/add-folder.png",
                    }
                }
                
                button { 
                    style: "position: relative; width: 22px; height: 22px; top: 1px; margin-right: 5px;",
                    title: "検索",
                    disabled: "{is_busy.read()}",
                    onclick: move |_| async move {
                        app.write().search_memos.clear();
                        app.write().search_result_message = String::new();
                        app.write().search_string = String::new();
                        app.write().state = app::State::Search; 
                    }, 
                    img {  
                        style:"position:relative; left: -3px; top: -1px;",
                        src: "https://img.icons8.com/color/16/search--v1.png",
                    }
                }

                button { 
                    style: "position: relative; width: 22px; height: 22px; top: 1px; margin-right: 5px;",
                    title: "クレジット",
                    disabled: "{is_busy.read()}",
                    onclick: move |_| async move { app.write().is_show_dialog = !app().is_show_dialog; }, 
                    img {  
                        style:"position:relative; left: -3px; top: -1px;",
                        src: "https://img.icons8.com/color/16/terms-and-conditions.png",
                    }
                }
                
            }
            hr { style: "position: fixed; top: 40px; left: 0px; width: 100%;" }
            dialog { 
                style: "z-index: 10;",
                open: "{app().is_show_dialog}",
                div {
                    style: "text-align: center;",
                    h3{"クレジット"}
                    hr {}
                    p {  
                        a { "このツールは " }
                        a {  
                            style: "width: 75%; height: 50%;",
                            href: "https://icons8.jp/icons",
                            "icons8.com"
                        },
                        a { " さんのアイコンを使用しております。" }
                    },
                    button { 
                        onclick: move|_| async move { app.write().is_show_dialog = !app().is_show_dialog; },
                        "閉じる"
                    }
                }
            }

            div { 
                style: "position:fixed; top: 45px; left:0px; text-align: center; width: calc(100% - 5px); height:calc(100% - 65px); overflow-x: hide; overflow-y: auto;",
                if is_busy() && app().tab_set.memos.len() == 0{
                    img{ 
                        style: "position:relative; left: 0px; top: 5px; margin-right: 5px;",
                        class: "rotating",
                        src: "https://img.icons8.com/external-others-inmotus-design/32/external-Loading-loaders-others-inmotus-design-18.png"
                    }
                }
                
                for (i, m) in app().tab_set.memos.iter().enumerate(){
                    textarea {
                        style: "width: calc(100% - 25px); white-space: pre-wrap; resize: none; field-sizing: content; font-family: Arial; box-shadow: 1px 1px 2px 0 rgba(0, 0, 0, 0.5) inset;",
                        placeholder: "{m.name}",
                        onmounted: move |cx| {
                            elements.write().push(cx.data());
                        },
                        oninput: move |e| {
                            app().tab_set.memos[i].write_memo(&e.value());
                        },
                        oncontextmenu: move|e|{
                            app.write().menu.is_show_menu = true;
                            app.write().menu.memo_path = app().tab_set.memos[i].path.to_string();
                            app.write().menu.pos_x = e.data().coordinates().client().x;
                            app.write().menu.pos_y = e.data().coordinates().client().y;
                        },
                        "{m.content}" 
                    }
                }
            }  
            if app().menu.is_show_menu{
                div {
                    style: r#"display: flex; flex-flow: column; position: absolute; left: {app().menu.pos_x}px; top: {app().menu.pos_y}px; 
                    padding: 5px; background-color: rgba(0,0,0,0.25);"#,
                    button {style:"margin: 2px;", onclick: move|_| app().menu.open_memo(), "メモを開く"}
                    button {style:"margin: 2px;", onclick: move|_| app().menu.open_folder(), "フォルダを開く"}
                    button {style:"margin: 2px;", onclick: move |_| async move { spawn(memo::delete_memo(app)); }, "メモを削除"}
                } 
            }
        }         
    }
}

pub fn edit_tab(mut app: Signal<app::App>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center;",
            h2{"タブを編集"}
        }
        div{
            style: "display:flex; flex-direction:column; align-items:center;",
            div{
                style: "",
                form {  
                    onsubmit: move|_|{
                        let b_path = format!("{}/{}", app::get_exe_path(), &app().tab_set.tabs[app().json.current_num].name);
                        let a_path = format!("{}/{}", app::get_exe_path(), &app().edit_tab.name);
                        let _ = std::fs::rename(&b_path, &a_path).unwrap(); 
                        app.write().state = app::State::Home;
                        app.write().is_reset_memo = true;
                    },
                    label{"タブ名:"}
                    input{
                        style: "width: 70vh;",
                        r#type: "text",
                        placeholder: "タブ名",
                        onmounted: move|cx|{let _ = cx.data().set_focus(true);  },
                        oninput: move |e| app.write().edit_tab.name = e.value(),
                        value: "{app().edit_tab.name}"
                    }
                }
            }
        }
        div{
            style: "display: flex; flex-direction: column; align-items: center; margin-top: 20px;",
            div{
                style: "",
                button {
                    onclick: move |_| {
                        let b_path = format!("{}/{}", app::get_exe_path(), &app().tab_set.tabs[app().json.current_num].name);
                        let a_path = format!("{}/{}", app::get_exe_path(), &app().edit_tab.name);
                        let _ = std::fs::rename(&b_path, &a_path).unwrap(); 
                        app.write().state = app::State::Home;
                        app.write().is_reset_memo = true;
                    },
                    "編集"
                }
                button{
                    onclick: move |_| {
                        
                        let res = native_dialog::MessageDialog::new()
                            .set_type(native_dialog::MessageType::Info)
                            .set_title("確認")
                            .set_text("このタブを削除しますか?")
                            .show_confirm().unwrap();
                        
                        if res {
                            let num = app().json.current_num;
                            let name = &app().tab_set.tabs[num].name;
                            let path = format!("{}/{}", app::get_exe_path(), name);
                            if std::path::Path::new(&path).is_dir(){
                                rm_rf::remove(&path).unwrap();
                                if app().json.current_num != 0{
                                    app.write().json.current_num -= 1;
                                }
                            }
                            app.write().is_reset_memo = true;
                            app.write().state = app::State::Home;
                        }
                    },
                    "削除"
                }
                button{
                    onclick: move |_| {
                        let res = native_dialog::MessageDialog::new()
                            .set_type(native_dialog::MessageType::Info)
                            .set_title("確認")
                            .set_text("メモをoldフォルダに移動させますか？")
                            .show_confirm().unwrap();
                        if res {
                            let c_path = app().tab_set.current_memo_path.to_string();
                            let root = std::path::Path::new(&c_path).parent().unwrap().display().to_string();
                            let old_path = format!("{}/old", root);
                            if !std::path::Path::new(&old_path).is_dir(){Some(std::fs::create_dir_all(&old_path));}
                            for m in app().tab_set.memos.iter(){
                                let base = m.path.to_string();
                                let f_name = std::path::Path::new(&base).file_name().unwrap().to_str().unwrap().to_string();
                                let target= format!("{}/{}",old_path, f_name);
                                let _res = std::fs::copy(&base, target);
                                let _res = std::fs::remove_file(&base);
                            }
                            app.write().is_reset_memo = true;
                            app.write().state = app::State::Home
                        }
                    },
                    "oldフォルダに移動"
                }

                button{
                    onclick: move |_| app.write().state = app::State::Home,
                    "戻る"
                }
            }
        }
    }
}

pub fn add_tab(mut app: Signal<app::App>, mut tab: Signal<app::Tab>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center;",
            h2{"タブを追加"}
        }
        div{
            style: "display: flex; flex-direction: column; align-items: center;",
            div{
                form {  
                    onsubmit: move|_|{
                        let exe_path = app::get_exe_path();
                        let new_dir = format!("{}/{}", &exe_path, tab().name);
                        if std::path::Path::new(&new_dir).is_dir(){
                            let _ = native_dialog::MessageDialog::new()
                                .set_type(native_dialog::MessageType::Info)
                                .set_title("確認")
                                .set_text("既にフォルダが存在します")
                                .show_alert();
                        }else{
                            Some(std::fs::create_dir_all(&new_dir));
                            tab.set(app::Tab::default());
                            app.write().state = app::State::Home;
                            app.write().is_reset_memo = true;
                        }
                    },
                    label{"タブ名:"}
                    input{
                        style: "width: 50vh;",
                        r#type: "text",
                        placeholder: "タブ名",
                        onmounted: move|cx|{let _ = cx.data().set_focus(true);  },
                        oninput: move |e| tab.write().name = e.value(),
                        value: "{tab().name}"
                    }
                }
            }
        }
        div{
            style: "display: flex; flex-direction: column; align-items: center; margin-top: 20px;",
            div{
                style: "",
                button{
                    onclick: move |_| {
                        let exe_path = app::get_exe_path();
                        let new_dir = format!("{}/{}", &exe_path, tab().name);
                        if std::path::Path::new(&new_dir).is_dir(){
                            let _ = native_dialog::MessageDialog::new()
                                .set_type(native_dialog::MessageType::Info)
                                .set_title("確認")
                                .set_text("既にフォルダが存在します")
                                .show_alert();
                        }else{
                            Some(std::fs::create_dir_all(&new_dir));
                            tab.set(app::Tab::default());
                            app.write().state = app::State::Home;
                            app.write().is_reset_memo = true;
                        }
                    },
                    "追加"
                }
                button{
                    onclick: move |_| app.write().state = app::State::Home,
                    "戻る"
                }
            }
        }
    }
}