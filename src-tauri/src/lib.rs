use serde::{Deserialize, Serialize}; // データのシリアライズとデシリアライズのためのクレートをインポート
use std::fs::File; // ファイル操作のための標準ライブラリをインポート
use std::io::{Read, Write}; // 読み込みと書き込み用のトレイトをインポート
use std::path::Path; // パス操作のための標準ライブラリをインポート

// メモのデータ構造を定義（id, title, content の3つのフィールドを持つ）
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Memo {
    id: u32,          // メモの一意な識別子
    title: String,    // メモのタイトル
    content: String,  // メモの内容
}

// 指定されたパスからデータを読み込む関数
fn read_data(file_path: &str) -> Vec<Memo> {
    let path = Path::new(file_path); // ファイルパスをPath型に変換

    // ファイルが存在しない場合、新しいファイルを作成して空のデータを返す
    if !path.exists() {
        File::create(file_path).expect("Failed to create file"); // ファイルを作成
        return Vec::new(); // 空のVecを返す
    }

    // ファイルを開き、内容を文字列として読み込む
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");

    // JSON文字列をデコードしてMemoのVecとして返す
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}

// データを指定されたパスに書き込む関数
fn write_data(file_path: &str, data: &Vec<Memo>) {
    // メモリ内のデータをJSON文字列に変換  ;fklsd
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize data");
    let mut file = File::create(file_path).expect("Failed to create file"); // ファイルを新規作成
    file.write_all(json.as_bytes()).expect("Failed to write file"); // データを書き込む
}

// 新しいメモを作成しファイルに保存する関数（Tauriコマンドとして公開）
#[tauri::command]
fn create_memo(file_path: &str, title: String, content: String) -> Memo {
    let mut data = read_data(file_path); // 既存のデータを読み込む
    let id = if let Some(last) = data.last() {
        last.id + 1 // データが存在する場合、最後のIDを取得して+1する
    } else {
        1 // データが空の場合、IDを1に設定
    };

    // 新しいメモを作成してデータに追加
    let memo = Memo { id, title, content };
    data.push(memo.clone());
    write_data(file_path, &data); // 更新されたデータを書き込む
    memo // 作成したメモを返す
}

// 指定されたIDのメモを取得する関数（Tauriコマンドとして公開）
#[tauri::command]
fn get_memo(file_path: &str, id: u32) -> Option<Memo> {
    let data = read_data(file_path); // データを読み込む
    data.into_iter().find(|memo| memo.id == id) // IDが一致するメモを探す
}

// 指定されたIDのメモを更新する関数（Tauriコマンドとして公開）
#[tauri::command]
fn update_memo(
    file_path: &str,
    id: u32,
    title: Option<String>,
    content: Option<String>,
) -> Result<Option<Memo>, String> {
    let mut data = read_data(file_path); // 既存のデータを読み込む

    // IDが一致するメモを見つけて更新
    let updated_memo = {
        if let Some(memo) = data.iter_mut().find(|memo| memo.id == id) {
            if let Some(new_title) = title {
                memo.title = new_title; // タイトルを更新
            }
            if let Some(new_content) = content {
                memo.content = new_content; // 内容を更新
            }
            Some(memo.clone()) // 更新されたメモを返す
        } else {
            None // メモが見つからなかった場合Noneを返す
        }
    };

    write_data(file_path, &data); // 更新されたデータを書き込む
    Ok(updated_memo) // 更新されたメモを結果として返す
}

// 指定されたIDのメモを削除する関数（Tauriコマンドとして公開）
#[tauri::command]
fn delete_memo(file_path: &str, id: u32) -> bool {
    let mut data = read_data(file_path); // データを読み込む
    let len_before = data.len(); // 削除前のデータの長さを取得
    data.retain(|memo| memo.id != id); // 指定されたID以外のメモを保持

    if data.len() < len_before {
        write_data(file_path, &data); // データを書き込む
        return true; // 削除成功
    }

    false // 削除対象が見つからなかった場合
}

// テスト用のモジュール
#[cfg(test)]
mod tests {
    use super::*; // モジュール内の関数をインポート

    const TEST_FILE_PATH: &str = "../data/test_data.json"; // テスト用のファイルパス

    // テスト用ファイルを初期化
    fn setup_test_file() {
        let _ = std::fs::remove_file(TEST_FILE_PATH); // ファイルを削除（存在する場合）
        let initial_data = serde_json::to_string(&Vec::<Memo>::new()).unwrap(); // 空のデータをJSON形式に
        std::fs::write(TEST_FILE_PATH, initial_data).unwrap(); // ファイルに書き込む
    }

    #[test]
    fn test_read_data() {
        setup_test_file(); // 初期化
        let data = read_data(TEST_FILE_PATH); // データを読み込む
        assert!(data.is_empty(), "初期状態ではデータは空"); // 空であることを確認
    }

    #[test]
    fn test_create_memo() {
        setup_test_file(); // 初期化
        let title = "TESTmemo".to_string();
        let content = "testcontent".to_string();
        let memo = create_memo(TEST_FILE_PATH, title.clone(), content.clone()); // メモを作成
        assert_eq!(memo.title, title); // タイトルが正しいか確認
        assert_eq!(memo.content, content); // 内容が正しいか確認

        let data = read_data(TEST_FILE_PATH);
        assert_eq!(data.len(), 1, "1件のメモが保存されます"); // メモが保存されたか確認
    }
}

// アプリケーションのエントリーポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // プラグインを初期化
        .invoke_handler(tauri::generate_handler![
            greet,
            create_memo,
            get_memo,
            update_memo,
            delete_memo
        ]) // コマンドを登録
        .run(tauri::generate_context!()) // アプリケーションを実行
        .expect("error while running tauri application");
}

// Tauriで呼び出せるシンプルな挨拶関数
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name) // 名前を含む挨拶メッセージを返す
}
