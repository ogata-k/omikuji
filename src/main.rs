extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use std::fs::create_dir;

fn main() {
    let s_vec: Vec<String> = std::env::args().collect();
    let data_folder = Path::new(&s_vec[0])
                    .canonicalize()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("data");
    println!("{:?}内のデータを読み込んでいます。", data_folder);
    
    let about_msg: &str = &format!("このプログラムはおみくじを一度引くだけのプログラムです。\nおみくじの内容は拡張子がomkjとなるファイル(以下omkjファイル)に空行無しで各行を0~6(0が最悪)の数字と伝えたいメッセージを空白区切りで指定できます。omkjファイルのファイル名が表示させたい名前になります。(例えば金運.omkj)\nomkjファイルは{:?}に配置してください。", data_folder);
    let app = App::new("omikuji")
        .version("0.0.1")
        .author("ogata-k <ogtkzk712@gmail.com>")
        .about(about_msg)
        .arg(Arg::with_name("yname")
                .help("あなたの名前")
                .short("n")
                .long("name")
                .takes_value(true)
                .default_value("") // デフォルトの場合は"〇〇さん"とせずに"あなた"とする。
            );
    
    let matches = app.get_matches();

    if let Err(e) = data_folder.read_dir(){
        println!("omkjファイルを入れるフォルダ{:?}が見つかりませんでした。作成します。", data_folder);
        create_dir(&data_folder).expect("フォルダの作成に失敗しました。");
        println!("作成しました。");
    }
    for entry_result in data_folder.read_dir().unwrap() {
        let entry = entry_result.unwrap();
        println!("{:?}というファイルを読み込んでいます…", entry.file_name());
    }

    let yname: &str = matches.value_of("yname").unwrap();
    let name: String = 
        if "" == yname{
            "あなた".to_string()
        } else {
           format!("{}さん", &yname.clone())
        };

    println!("{}の運勢はhoge", &name);
}
