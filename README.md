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

## インストーラー方式(自動セットアップ)

利用者はopen-tv-chatクライアントをダウンロードするだけで使えるようにする。
`open-web-server`のインストール・自分の端末上でのWEBサーバー起動を、利用者に
手動作業をさせず**インストーラーが自動で行う**方針とする。

1. open-tv-chatインストーラー起動 → `open-web-server`本体を同梱物または自動
   ダウンロードで導入(サイレントインストール)。
2. `open-web-server`をローカル専用設定(後述のセキュリティ方針)で自動起動・
   自動設定。
3. open-tv-chat本体をインストール、初回起動時にローカルの`open-web-server`へ
   自動接続。

利用者が意識するのは「open-tv-chatをダウンロードして起動する」だけで完結させる
(`open-web-server`の存在・設定を利用者に意識させない)。具体的なインストーラー
実装(サイレントインストールのオプション・権限昇格の扱い等)は実装着手時に
`open-web-server`側リポジトリと合わせて設計する。

## セキュリティ/プライバシー方針(端末情報の非漏洩)

自動インストールされる`open-web-server`が、利用者の端末情報を外部に漏らさない
ことを設計上の必須要件とする。

- **待受はローカルのみ**: 自動セットアップされる`open-web-server`は既定で
  `127.0.0.1`(または同等のプライベートインターフェース)にのみバインドし、
  インターネットから直接到達可能なポート開放・UPnP/NAT越えの自動設定は行わない。
- **シグナリングは最小情報のみ**: 通話の呼び出し・ルーティングに必要な最小限の
  情報(セッションID・公開鍵等)のみを中継サーバー側とやり取りし、IPアドレス・
  端末識別子・OSバージョン・ファイルパス等の端末固有情報は送信しない。
- **テレメトリ既定オフ**: 利用状況・クラッシュレポート等の送信はデフォルトで
  無効化し、送信する場合は利用者の明示的なオプトインを必須とする。
- **通信は暗号化必須**: クライアント⇔ローカル`open-web-server`間・
  `open-web-server`⇔通話相手間はいずれもTLS/暗号化を必須とし、平文通信を
  許可しない。
- **ログの最小化**: ローカルログに端末情報(IPアドレス・MACアドレス・
  ユーザー名等)を平文で残さない。デバッグ目的のログも既定では無効、または
  マスキングした形式のみとする。
- **アンインストール時の後始末**: `open-web-server`のアンインストール時に、
  設定・鍵・ログ等の残存データを確実に削除する(または利用者に削除するか選択させる)。

具体的な実装(証明書管理・鍵交換方式・ファイアウォール設定の自動化範囲等)は、
`open-web-server`側の既存の4層防御通信方針と整合させながら実装着手時に詳細化する。

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
