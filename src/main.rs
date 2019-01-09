extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = App::new("omikuji")
        .version("0.0.1")
        .author("ogata-k <ogtkzk712@gmail.com>")
        .about("このプログラムはおみくじを一度引くだけのプログラムです。data直下にomkjファイルを空行無しで各行を0~6(0が最悪)の数字と伝えたいメッセージを空白区切りで指定できます。omkjファイルのファイル名が表示させたいなまえになります。(例えば金運.omkj)")
        .arg(Arg::with_name("yname")
                .help("あなたの名前")
                .short("n")
                .long("name")
                .takes_value(true)
                .default_value("") // デフォルトの場合はさんを付けずにあなたとする。
            );
    
    let matches = app.get_matches();

    let yname: &str = matches.value_of("yname").unwrap();
    let name: String = 
        if "" == yname{
            "あなた".to_string()
        } else {
           format!("{}さん", &yname.clone())
        };

    println!("{}の運勢はhoge", &name);
}

