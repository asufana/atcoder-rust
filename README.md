## 環境整備
```sh
# .devcontainer 環境から
cargo compete login atcoder

# ログインできない場合には下記を試してみる
# https://zenn.dev/stddev/articles/388b4d2acb248e
```

## 課題処理
```sh
cd app
./add.sh abc086
cd abc086
../test.sh a
../submit.sh a
```