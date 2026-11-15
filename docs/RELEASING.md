# リリース手順

このワークスペースは 5 つのクレートを crates.io に公開します。

| クレート | 内容 | 依存 |
| --- | --- | --- |
| `vmix-core` | データ構造と XML パース | — |
| `vmix-tcp` | TCP API クライアント | vmix-core |
| `vmix-shortcuts` | ショートカットカタログ（埋め込み JSON） | — |
| `vmix-http` | HTTP API クライアント | vmix-core, vmix-tcp |
| `vmix-rs` | 上記をまとめるクレート | すべて |

## 1. バージョンを上げる

`Cargo.toml` の `[workspace.package] version` と、クレート間の `path` 依存に書かれたバージョンを揃えて更新します。

```toml
# Cargo.toml
[workspace.package]
version = "0.2.3"
```

```toml
# vmix-tcp/Cargo.toml, vmix-http/Cargo.toml, vmix-rs/Cargo.toml
vmix-core = { path = "../vmix-core", version = "0.2.3", features = ["std", "xml"] }
```

`crates.io` に公開済みのバージョンは上書きできないため、必ず新しいバージョンにしてください。

## 2. マージしてタグを push する

```bash
# main にマージした後
git tag -a v0.2.3 -m "Release v0.2.3"
git push origin v0.2.3
```

`v*` タグの push が `.github/workflows/publish.yml` を起動します。ワークフローは次の順に処理します。

1. タグ名と `Cargo.toml` のバージョンが一致しているか検証
2. `cargo fmt --check` / `cargo clippy --all-features` / `cargo test --all-features`
3. 依存順に `vmix-core` → `vmix-tcp` → `vmix-shortcuts` → `vmix-http` → `vmix-rs` を公開
4. GitHub Release を自動作成（`--generate-notes`）

公開済みのバージョンはスキップするので、途中で失敗しても**同じタグのまま再実行**すれば続きから進みます。

```bash
gh run list -R MikanseiLaboratory/vmix-rs --workflow Publish
gh run rerun <run-id> -R MikanseiLaboratory/vmix-rs --failed
```

## 3. crates.io の認証

ワークフローは 2 つの方式に対応しています。**どちらか一方**を設定してください。

### A. Trusted Publishing（推奨・トークン不要）

crates.io のクレート設定で、クレートごとに trusted publisher を登録します。

- Provider: `GitHub`
- Repository: `MikanseiLaboratory/vmix-rs`
- Workflow file: `publish.yml`
- Environment: 空欄

設定ページ:

- https://crates.io/crates/vmix-core/settings
- https://crates.io/crates/vmix-tcp/settings
- https://crates.io/crates/vmix-shortcuts/settings
- https://crates.io/crates/vmix-http/settings
- https://crates.io/crates/vmix-rs/settings

> **注意**: 発行される一時トークンの権限は「リポジトリ / ワークフロー名 / environment が一致した設定のクレート」に限定されます。1 クレートでも登録が漏れると、そのクレートの公開だけが `403 Forbidden: The provided access token is not valid for crate ...` で失敗します。

### B. リポジトリシークレット `CARGO_REGISTRY_TOKEN`

`publish-new` と `publish-update` スコープを持つ crates.io の API トークンを登録します。

```bash
gh secret set CARGO_REGISTRY_TOKEN -R MikanseiLaboratory/vmix-rs
```

シークレットが設定されている場合はこちらが優先され、Trusted Publishing は使われません。Trusted Publishing に完全移行するときは、5 クレートすべての設定を済ませてからシークレットを削除してください。

```bash
gh secret delete CARGO_REGISTRY_TOKEN -R MikanseiLaboratory/vmix-rs
```

### 新規クレートの初回公開

crates.io は「まだ存在しないクレート」に Trusted Publishing を設定できません。新しいクレートを追加したときの初回だけは、`CARGO_REGISTRY_TOKEN`（`publish-new` スコープ付き）での公開が必要です。初回公開が終われば、そのクレートにも trusted publisher を登録できます。

## 4. 公開後の確認

```bash
# crates.io のバージョン
curl -s https://crates.io/api/v1/crates/vmix-rs | jq -r .crate.max_stable_version

# docs.rs のビルド結果（true なら成功）
curl -s https://docs.rs/crate/vmix-rs/0.2.3/status.json

# GitHub Release
gh release view v0.2.3 -R MikanseiLaboratory/vmix-rs
```
