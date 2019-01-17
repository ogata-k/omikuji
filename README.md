# omikuji
ファイルを読み込んで表示するおみくじ

このプログラムはおみくじを一度引くだけのプログラムです。
おみくじの内容は拡張子がomkjとなるファイル(以下omkjファイル)に空行無しで各行を0~6(0が最悪)の数字と伝えたいメッセージを空白区切りで指定できます。omkjファイルのファイル名が表示させたい名前になります。(例えば金運.omkj)
omkjファイルは"/home/ogata/dev/Rust/omikuji/executable/x86_64-unknown-linux-gnu/data"に配置してください。

USAGE:
    omikuji [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <yname>    あなたの名前 [default: ]
