## 概要
このリポジトリは、Rustで作ったメモツールのリポジトリになります

<img width="409" height="248" alt="image" src="https://github.com/user-attachments/assets/4f80c9a4-3a77-4269-816c-0c5759f190b4" />

## ビルド
[こちら](https://dioxuslabs.com/learn/0.6/guide/tooling#setting-up-tooling)を参考にしてください。
```
git clone https://github.com/8bitTD/memo_dioxus
cd memo_dioxus
dx serve --release
```

## 使い方
　memo_dioxus.exeをダブルクリックするとGUIが表示されます<br>
　<img width="488" height="293" alt="image" src="https://github.com/user-attachments/assets/aa38f0b1-43e1-44d8-a5b6-b6a80b34e2c9" />

### タブを作成 (ctrl + t)
　<img width="312" height="138" alt="image" src="https://github.com/user-attachments/assets/d888eee4-ad8a-45d1-9d7f-96194be7a124" />

　GUI右上の"タブを追加"ボタンを押すとタブを追加画面になり、タブ名を入力して追加ボタンを押してください<br>
　GUI上部タブが作成され、memo_dioxus.exe直下に同名のフォルダが作成されます<br>
　
### メモを作成 (ctrl + n)
　作成されたタブを選択してアクティブ状態にし、GUI右上の"メモを作成"ボタンを押してください<br>
　memo_dioxus.exe直下のタブフォルダに年月日時分秒_タブ名.txtファイルが作成され、メモが入力できるようになります<br>
　　
### タブを編集
　<img width="415" height="139" alt="image" src="https://github.com/user-attachments/assets/4f25cc00-a478-40ca-80f7-8f0bf7bce02d" /><br>
　GUI右上の"タブを編集"ボタンを押すとタブ編集画面になります<br>
#### 編集
　タブ名を変更した状態で編集ボタンを押すとタブ名が変更されます<br>
#### 削除
　タブを削除します<br>
#### oldフォルダに移動
　タブ内のすべてのメモをoldフォルダに移動します。ツールの挙動が重くなってきたらoldフォルダに移動させてください。<br>
#### 戻る
　もとの画面に戻ります<br>
　　　
## 検索
　<img width="441" height="45" alt="image" src="https://github.com/user-attachments/assets/215e5988-5b22-489d-b82e-061b28cc0d7b" /><br>
　GUI右上の"検索"ボタンを押すとmemo_dioxus.exe階層内のすべてのメモを検索します<br>
　　
## クレジット
　<img width="502" height="180" alt="image" src="https://github.com/user-attachments/assets/6f4fdacb-37a6-4c65-9d64-419c8bfb2fbe" /><br>
　GUI右上の"クレジット"ボタンを押すとアイコンを使用させていただいている[icons8](https://icons8.jp/icons)様のページに飛べるダイアログが表示されます<br>
　こちらのライセンス条件を満たすためのものです<br>
　https://icons8.jp/license<br>
　　

## メモ上で右クリック
　メモの入力フィールド上でマウスの右クリックを押すとメニューが表示されます<br>
　<img width="121" height="107" alt="image" src="https://github.com/user-attachments/assets/7a99ee6a-ff36-4a98-b007-0a3c02fe9b6d" /><br>
### メモを開く
　メモをテキストエディターで開きます<br>
### フォルダを開く
　現在のタブフォルダをファイルエクスプローラで開きます<br>
### メモを削除
　メモが削除されます<br>

## 動作確認
　Windows10
