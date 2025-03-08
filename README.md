# ntdis

`ntdis`は、ローカルネットワーク上のmDNSサービスを発見するためのコマンドラインツールです。
また、このツールはadbコマンドに対するユーティリティを持ち合わせており、connect/pairを自動検索して実行することができ、左記以外のadbコマンドは対話的に接続されたデバイスを選択して実行することができます。

## インストール

このプロジェクトはRustで書かれており、Cargoを使用してビルドおよび実行できます。

```bash
cargo build --release
```

直接/local/binに配置する場合は以下のコマンドを実行してください。

```bash
curl -L https://github.com/yomifrogs/ntdis/releases/download/v0.1.0/ntdis -o /usr/local/bin/ntdis && chmod +x /usr/local/bin/ntdis
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

## 例

- ローカルネットワーク上のすべてのmDNSサービスをスキャンする:
  ```bash
  ntdis scan -v -d 5
  ```

- 特定のデバイスタイプをスキャンして接続する:
  ```bash
  ntdis adb connect -s 10
  ```

## ライセンス

このプロジェクトはMITライセンスの下で提供されています。
