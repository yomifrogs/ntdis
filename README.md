# ntdis

`ntdis`は、ローカルネットワーク上のmDNSサービスを発見するためのコマンドラインツールです。
また、このツールはadbコマンドに対するユーティリティを持ち合わせており、connect/pairを自動検索して実行することができ、左記以外のadbコマンドは対話的に接続されたデバイスを選択して実行することができます。

## インストール

このプロジェクトはRustで書かれており、Cargoを使用してビルドおよび実行できます。

```bash
cargo build --release
```

リポジトリをクローンする場合、もっとも簡単なインストール方法はCargoでインストールを実行することです

```bash
sh scripts/install.sh
```

リポジトリをクローンせず、直接/local/binに配置する場合は以下のコマンドを実行してください。

```bash
curl -L https://github.com/yomifrogs/ntdis/releases/download/v${latest_release_version}/ntdis -o /usr/local/bin/ntdis && chmod +x /usr/local/bin/ntdis
```

## 使い方

### 基本コマンド

```bash
ntdis --help
```

```text
ローカルネットワーク上のmDNSサービスを発見します

使用法: ntdis <COMMAND>

コマンド:
  scan  mDNSサービス発見のスキャンを実行します
  adb   adbコマンドを実行します
  help  このメッセージまたは指定されたサブコマンドのヘルプを表示します

オプション:
  -h, --help     ヘルプを表示します
  -V, --version  バージョンを表示します
```

### scanコマンド

```bash
ntdis scan --help
```

```text
mDNSサービス発見のスキャンを実行します

使用法: ntdis scan [OPTIONS]

オプション:
  -v, --verbose                      詳細な出力を有効にします
  -d, --duration <duration>          サービス発見のスキャン時間を秒単位で設定します [デフォルト: 3]
  -t, --service_type <service_type>  発見するサービスタイプを指定します
  -h, --help                         ヘルプを表示します
```

### adbコマンド

```bash
ntdis adb --help
```

```text
adbコマンドを実行します

使用法: ntdis adb [args]... [COMMAND]

コマンド:
  connect  デバイスに接続します
  pair     デバイスとペアリングします
  help     このメッセージまたは指定されたサブコマンドのヘルプを表示します

引数:
  [args]...  adbに渡す引数

オプション:
  -h, --help  ヘルプを表示します
```

#### adb connect

```bash
ntdis adb connect --help
```

```text
デバイスに接続します

使用法: ntdis adb connect [OPTIONS]

オプション:
  -s, --scan_duration <scan_duration>  サービス発見のスキャン時間を秒単位で設定します [デフォルト: 3]
      --auto                           デバイスが1台のみ見つかった場合、自動的に選択します
  -h, --help                           ヘルプを表示します
```

#### adb pair

```bash
ntdis adb pair --help
```

```text
デバイスとペアリングします

使用法: ntdis adb pair [OPTIONS]

オプション:
  -s, --scan_duration <scan_duration>  サービス発見のスキャン時間を秒単位で設定します [デフォルト: 3]
      --auto                           デバイスが1台のみ見つかった場合、自動的に選択します
  -h, --help                           ヘルプを表示します
```

## Example

### scan

- ローカルネットワーク上のすべてのmDNSサービスをスキャンする:

  ```bash
  ntdis scan
  ```

### adb

- adbでペアリングする
  - Android端末上で「ペア設定コードによるデバイスのペア設定」を選択し、ダイアログが開いた状態で下記を実行する
  - 検出されたデバイスを選択し、端末に表示されているペアコードを入力すれば、ペア設定ができる

  ```bash
  ntdis adb pair
  ```

- adbでペアリングする（自動選択）
  - デバイスが1台のみ検出された場合、自動的に選択される

  ```bash
  ntdis adb pair --auto
  ```

- adbで接続する
  - あらかじめペア設定を行っておくことが必要である

  ```bash
  ntdis adb connect
  ```

- adbで接続する（自動選択）
  - デバイスが1台のみ検出された場合、自動的に選択される

  ```bash
  ntdis adb connect --auto
  ```

- adbコマンドを実行する

  ```bash
  ntdis adb shell
  ```

## ライセンス

このプロジェクトはMITライセンスの下で提供されています。
