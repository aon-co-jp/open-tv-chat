# 開発方針＆開発環境ルール(open-tv-chat)

作業ドライブは`F:\open-tv-chat`。この節は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を
**正本**とし、各プロジェクトへコピーして同期する既存の運用ルール継承方針に
準じる(比較的新しいフレームワークの参照資料一覧・AI駆動開発ツールに関する
所感・確認不要の自動継続/リミット解除後の自動再開・白画面バグ等を見逃さない
検証徹底、等の全リポジトリ共通ルールは、詳細をここに複製せず
`open-raid-z/CLAUDE.md`を参照すること)。

## このリポジトリの役割

Skype風「世界約130ヶ国語 リアルタイム音声翻訳」対応のTV会議/ビデオチャット
アプリケーション。Windows/macOS/Linux/Android/iPhone向けクライアントを
easy-web.tokyoから配布し、利用者は自分のPCに`open-web-server`を立てて使う
構成を基本形とする。詳細な構想は[`README.md`](README.md)を参照。

## 技術方針

- サーバーサイド: Rust + [`RPoem`](https://github.com/aon-co-jp/RPoem)
  (WunderGraph Cosmo互換、REST API不要、Tomcat互換)。
- Webサーバー: [`open-web-server`](https://github.com/aon-co-jp/open-web-server)
  (Apache/Nginx互換)。
- 音声翻訳エンジン: 未定。ASR/MT/TTSの組み合わせ方針(自前実装/OSS/外部API)の
  調査結果は本ファイルおよび`PORTING.md`に追記していく。
- 同時翻訳の上限: 2ヶ国語間から開始し、希望に応じて最大10ヶ国語まで同時翻訳
  対応(N対N配信の帯域・レイテンシ設計は実装時に別途検討)。

## 対応言語のネイティブ表記ルール

言語一覧・UIの言語選択メニュー等では、必ず「英語名(現地呼称) = ネイティブ表記」
を併記する。ネイティブ表記を推測で埋めず、不明な場合は空欄のまま次回セッションの
確認事項として残すこと。例: Iran (Persia) = فارسی (Farsi)。

## 配布・インストーラー方針

各OS向けインストーラーの構成は、既存の
[`open-english`](https://github.com/aon-co-jp/open-english)のWindows/Android版
インストーラー・アンインストーラー・バージョン管理の実装を参考にする
(このリポジトリへ実装を複製せず、参照のみ)。

## HANDOFF

- **2026-09-26 リポジトリ新設**: `aon-co-jp/open-tv-chat`を新規作成し、
  設計ドキュメント(README.md/CLAUDE.md/PORTING.md)のみを整備した段階。
  実装(クライアント本体・サーバーサイド・音声翻訳エンジン)は未着手。
  次回再開時は[`PORTING.md`](PORTING.md)の「次回再開ポイント」を参照。
