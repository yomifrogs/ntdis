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

- `ntdis scan`: ローカルネットワーク上のmDNSサービスをスキャンします。
  - `-v`, `--verbose`: 詳細な出力を有効にします。
  - `-d`, `--duration <秒>`: サービス発見のスキャン時間を設定します（デフォルトは3秒）。
  - `-t`, `--service_type <タイプ>`: 発見するサービスのタイプを指定します。

- `ntdis adb connect`: デバイスに接続します。
  - `-s`, `--scan_duration <秒>`: サービス発見のスキャン時間を設定します（デフォルトは3秒）。

- `ntdis adb pair`: デバイスとペアリングします。
  - `-s`, `--scan_duration <秒>`: サービス発見のスキャン時間を設定します（デフォルトは3秒）。

- `ntdis adb <args>`: ADBコマンドを実行します。`args`はADBに渡す引数です。

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

- adbで接続する
  - あらかじめペア設定を行っておくことが必要である

  ```bash
  ntdis adb connect
  ```

- adbコマンドを実行する

  ```bash
  ntdis adb shell
  ```

## ライセンス

このプロジェクトはMITライセンスの下で提供されています。
