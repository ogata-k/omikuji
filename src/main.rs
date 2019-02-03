extern crate clap;
extern crate rand;

use std::io::Read;
use std::path::Path;
use std::fs::{create_dir, File};
use std::fmt;
use clap::{App, Arg};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

struct OmkjData{
  belong: String,
  eval: u8,
  msg: String,
}

impl fmt::Display for OmkjData{ 
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.belong, self.msg)
  }
}

fn main() {
    let s_vec: Vec<String> = std::env::args().collect();
    let data_folder = Path::new(&s_vec[0])
                    .canonicalize()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("data");
    // println!("{:?}内のデータを読み込んでいます。", data_folder);
    
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
    let mut omkj_datas: Vec<OmkjData> = Vec::new();
    let mut rng = rand::thread_rng();

    for entry_result in data_folder.read_dir().expect("ディレクトリを読み込めませんでした。") {
        let entry = entry_result.unwrap();

        // println!("{:?}というファイルを読み込んでいます…", entry.file_name());
        let mut file = File::open(data_folder.join(Path::new(&entry.file_name()))).expect("ファイルが開けませんでした。");
        let mut pre_contents = String::new();
        file.read_to_string(&mut pre_contents).expect("ファイルを読み込めませんでした。");
        // ファイルがからの場合の処理を指定内
        let contents: Vec<&str> = pre_contents.split('\n').filter(|&s| s != "").collect();

        //println!("{:?}", contents);

        //行選択用乱数
        let n: usize = Uniform::new(0, contents.len()).sample(&mut rng);

        let content_split: Vec<&str> = contents[n].trim().split_whitespace().collect();
        // ここでomkj_datasにセット
        let eval: u8 = (&content_split[0]).parse().unwrap();
        let msg: String = (&content_split[1 ..]).join(" ");
        // println!("{}: {}", eval, msg);
        let omkj_data = OmkjData{
belong: if let Some(filename) = Path::new(&entry.file_name()).to_string_lossy().to_owned().to_string().split('/').collect::<Vec<&str>>().last(){
            (filename.split('.').collect::<Vec<&str>>())[0].to_string()
        }
        else {"".to_string()}
        ,
          eval: eval,
          msg: msg
        };
        //println!("{}",omkj_data);
        omkj_datas.push(omkj_data);
        //println!("取得に成功しました。");
    }

    let yname: &str = matches.value_of("yname").unwrap();
    let name: String = 
        if "" == yname{
            "あなた".to_string()
        } else {
           format!("{}さん", &yname.clone())
        };

    println!("{}の運勢は…", &name);
    for omkj_data in &omkj_datas{
      std::thread::sleep(std::time::Duration::from_millis(2000));
      println!("{}", omkj_data);
    }
    let e_sum: u8 = omkj_datas.iter().map(|data| data.eval).sum();
    let e: u8 = e_sum / (omkj_datas.len() as u8);

    println!("\nよって…");
    std::thread::sleep(std::time::Duration::from_millis(3000));
    //println!("{} {}", e_sum, e);
    println!("{}", get_msg(e));
}

fn get_msg(n: u8) -> String{
  match n {
    0 => "大凶".to_string(),
    1 => "凶".to_string(),
    2 => "末吉".to_string(),
    3 => "吉".to_string(),
    4 => "小吉".to_string(),
    5 => "中吉".to_string(),
    6 => "大吉".to_string(),
    _ => "???".to_string()
  }
}
