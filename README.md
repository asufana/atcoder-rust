
# 環境構築
### 参考
https://it.commutty.com/shunnya0715/articles/9734ba4922894b329a756b6f3592e8ed

### 環境
```shell
$ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/hana/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
1.70.0-aarch64-apple-darwin (default)
1.75.0-aarch64-apple-darwin

active toolchain
----------------

1.70.0-aarch64-apple-darwin (default)
rustc 1.70.0 (90c541806 2023-05-31)
```

# スクリプト
### コンテスト追加
```shell
$ ./add.sh ABC100 && cd $_
```

### テスト
```shell
$ ../test.sh a
```

### サブミット
```shell
$ ../submit.sh a
```