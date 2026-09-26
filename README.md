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

## 音声翻訳エンジンの選定(調査完了・2026-09-26)

商用配布(easy-web.tokyoでの無償/有償配布)を前提に、ASR(音声認識)は
[Whisper](https://github.com/openai/whisper)(MIT、約99言語)、MT(機械翻訳)は
[MADLAD-400](https://huggingface.co/google/madlad400-3b-mt)(Apache-2.0、400+言語)を
軸とする構成を第一候補とした。Meta製のSeamlessM4T/SeamlessStreaming/MMSは言語数・
性能で優位だがCC-BY-NC 4.0(非商用限定)のため製品には採用しない。TTSは
[Piper](https://github.com/OHF-Voice/piper1-gpl)を候補としつつ、声モデルごとに
ライセンスが異なるため個別確認が必要。詳細な比較表・方針は
[`PORTING.md`](PORTING.md)「1. 音声翻訳エンジン調査結果」を参照。

## 多言語表記ルール

対応言語一覧は「英語名 (現地呼称のローマ字表記) = ネイティブ表記 (現地語での言語名)」
の形式で統一する。例:

- Iran (Persia) = فارسی (Farsi)
- Japan = 日本語 (Nihongo)
- France = Français (French)

正式な対応言語一覧(約130ヶ国語分)はISO 639-1/639-3をベースに実装時に機械的に
生成し、上記の表記ルールに沿って人手で現地語ネイティブ表記を確認しながら整備する
(誤ったネイティブ表記を推測で埋めない)。サンプル30件(要ネイティブ話者検証)は
[`PORTING.md`](PORTING.md)「4. 対応言語リスト」に掲載。

## 開発方針

このリポジトリの開発ルールは[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の
`CLAUDE.md`を正本とする、`aon-co-jp`エコシステム共通の運用ルール継承方針に従う。
詳細は[`CLAUDE.md`](CLAUDE.md)を参照。

## 現在の到達点

2026-09-26時点: リポジトリ雛形・設計ドキュメント作成に加え、音声翻訳エンジン調査
(ASR: Whisper / MT: MADLAD-400 / TTS: Piper候補)、クライアント実装方式
(Rust + Tauri候補)、easy-web.tokyo配布導線、対応言語サンプル30件までの方針案を
整備した。コード実装(クライアント本体・サーバーサイド)は未着手。詳細・次回再開
ポイントは[`PORTING.md`](PORTING.md)を参照。
