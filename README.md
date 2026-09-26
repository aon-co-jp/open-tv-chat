# open-tv-chat

Skype風「世界約130ヶ国語 リアルタイム音声翻訳」対応の
TV会議/ビデオチャットアプリケーション。**現時点は設計ドキュメント段階**であり、
実装(Windows/macOS/Linux/Android/iPhone向けクライアント本体)はまだ着手していない。

## 目的

- [easy-web.tokyo](https://easy-web.tokyo)上で紹介ページを公開し、各OS向け
  クライアントをダウンロードしてもらう。
- 利用者は自分のPCに[`open-web-server`](https://github.com/aon-co-jp/open-web-server)
  (Apache/Nginx互換のRust製Webサーバー)をインストールし、自前でWEBサーバーを
  立てて使う構成を基本形とする。
- サーバーサイドは Rust + [`RPoem`](https://github.com/aon-co-jp/RPoem)
  ([WunderGraph Cosmo](https://github.com/wundergraph/cosmo)互換のGraphQL
  Federation実装。REST API不要、Tomcat互換)を利用する。
- 音声通話中に、旧Microsoft Skypeのリアルタイム翻訳機能のように、世界約130ヶ国語
  間の音声翻訳を行う。2ヶ国語間の翻訳から開始し、希望すれば最大10ヶ国語まで
  同時に翻訳できるようにする。

## 対応クライアント(予定)

| プラットフォーム | 状態 |
|---|---|
| Windows | 未着手 |
| macOS | 未着手 |
| Linux | 未着手 |
| Android | 未着手 |
| iPhone (iOS) | 未着手 |

配布はいずれも easy-web.tokyo からのダウンロード形式を想定(インストーラー形式は
[`open-english`](https://github.com/aon-co-jp/open-english)のWindows/Android版
インストーラーの構成を参考にする)。

## アーキテクチャ概要(構想)

```
[クライアント: Win/Mac/Linux/Android/iPhone]
        │ (音声/映像 + 翻訳字幕・音声)
        ▼
[利用者PC上の open-web-server] ── Apache/Nginx互換、TLS終端
        │
        ▼
[Rust + RPoem] ── Cosmo互換GraphQL Federation、REST API不要、Tomcat互換
        │
        ▼
[音声翻訳エンジン] ── 未定(下記「音声翻訳エンジンの選定」参照)
```

- 通信の暗号化・可用性設計は、既存のミッションクリティカル向け方針
  ([`open-web-server`](https://github.com/aon-co-jp/open-web-server)の
  4層防御通信)を踏襲する。
- 認証・課金・データ保全の考え方は今後 `PORTING.md` / `CLAUDE.md` で詳細化する。

## 音声翻訳エンジンの選定(未定・調査中)

130ヶ国語・リアルタイム同時通訳という要件を満たすASR(音声認識)+MT(機械翻訳)+
TTS(音声合成)の組み合わせは、自前実装/OSS/外部API混在のいずれになるか未確定。
候補の調査・比較は次回セッション以降の課題とする(既存資産の
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm)の自前LLM基盤を
活用できるかも含めて検討する)。

## 多言語表記ルール

対応言語一覧は「英語名 (現地呼称のローマ字表記) = ネイティブ表記 (現地語での言語名)」
の形式で統一する。例:

- Iran (Persia) = فارسی (Farsi)
- Japan = 日本語 (Nihongo)
- France = Français (French)

正式な対応言語一覧(約130ヶ国語分)はISO 639-1/639-3をベースに実装時に機械的に
生成し、上記の表記ルールに沿って人手で現地語ネイティブ表記を確認しながら整備する
(誤ったネイティブ表記を推測で埋めない)。

## 開発方針

このリポジトリの開発ルールは[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とする、`aon-co-jp`エコシステム共通の運用ルール継承方針に従う。
詳細は[`CLAUDE.md`](CLAUDE.md)を参照。

## 現在の到達点

2026-09-26時点: リポジトリ雛形作成・設計ドキュメント(本README/CLAUDE.md/PORTING.md)
のみ。実装は未着手。次回以降の再開ポイントは[`PORTING.md`](PORTING.md)を参照。
