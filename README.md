# open-tv-chat

Skype風「世界約130ヶ国語 リアルタイム音声翻訳」対応の
TV会議/ビデオチャットアプリケーション。**現時点は設計ドキュメント段階**であり、
実装(Windows/macOS/Linux/Android/iPhone向けクライアント本体)はまだ着手していない。

## 目的

- [easy-web.tokyo](https://easy-web.tokyo)上で紹介ページを公開し、各OS向け
  クライアントをダウンロードしてもらう。
- **既定はP2P通信とする(2026-09-27方針、4回目の改訂)**。自分の端末に
  サーバーを立てないことで、ローカルの機器・データを外部に晒すリスクを
  減らす。`open-web-server`を自分の端末に立てて使う構成は**オプション**
  (P2Pが繋がりにくい環境向けの代替、または上級者向け)という位置づけに
  変更した。下記「アーキテクチャ概要」参照。
- どちらのモードでも、相手や外部から自分の端末の真のIPアドレスが見えない
  よう、[`open-LiveKit`](https://github.com/aon-co-jp/open-LiveKit)
  (LiveKit参考のRust製SFU/リレー、Rust + [`RPoem`](https://github.com/aon-co-jp/RPoem)、
  [WunderGraph Cosmo](https://github.com/wundergraph/cosmo)互換のGraphQL
  Federation)を経由する**二段階の中継(踏み台)**を運営側(aon-co-jp)が
  用意する(利用者端末へのインストールは不要)。
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

## アーキテクチャ概要(構想・2026-09-27 「P2P既定+二段階踏み台、サーバーはオプション」方式に決定・4回目の改訂)

**既定はP2Pモード**。自分の端末にサーバーを立てないことで、ローカルの
機器・データを外部に晒すリスクを避ける。`open-web-server`を自分の端末に
立てて通話ホストになる「サーバーモード」は**オプション**(P2Pが繋がりにくい
環境向けの代替、または上級者向け)という位置づけに変更した。どちらのモード
でも、真のIPアドレスが相手や外部に漏れないよう、
[`open-LiveKit`](https://github.com/aon-co-jp/open-LiveKit)による
二段階の中継(踏み台)を経由させる(検討の結果、新規の汎用VPNアプリは作らず、
既存のSFUリレー設計を拡張する「選択肢A」を採用、詳細はPORTING.md
「8. 接続モード方針の再変更」参照)。

```
[利用者端末A]                                          [利用者端末B]
Win/Mac/Linux/Android/iPhone                          Win/Mac/Linux/Android/iPhone
   │
   ├─ ① 既定 = 「P2Pモード」(自分の端末にサーバーを立てない)
   │     ※ P2Pが繋がりにくい環境向けの代替、または利用者が明示的に
   │       選んだ場合のみ「サーバーモード」(自分でopen-web-serverを起動)
   │
   └────────────┐                              ┌────────────┘
                ▼                              ▼
        [踏み台Hop1]  ──────────────────  [踏み台Hop2]
      (入口側。接続元の真のIPは見えるが、
       最終的な相手/内容までは見えない)   (出口側。相手側と通信するが、
                                            接続元の真のIPは見えない)
                │                              │
                └──────────► [音声翻訳エンジン(Agentフック)] ◄──────────┘
                          open-LiveKit(webrtc-rs + RPoem)上で処理
```

- **P2Pモード/サーバーモードいずれも、二段階踏み台を経由するのが既定**。
  サーバーモードを選んだ場合も、その公開IPをそのままインターネットに晒さず、
  踏み台の向こうに隠す。
  (WebRTCで一般的な1段リレー/TURNと異なり、**入口(Hop1)と出口(Hop2)を
  別ノードに分離**することで、単一の中継ノードが「誰が・誰と話しているか」を
  同時に把握できないようにする、簡易な二段階中継=「踏み台」構成とする)。
- 翻訳(ASR/MT/TTS)は、二段階踏み台を経由した先の`open-LiveKit`側で
  一括処理する(参加者の端末スペックに依存しにくく、最大10ヶ国語同時翻訳との
  相性が良い、という以前の方針を維持)。
- 通信の暗号化・可用性設計は、既存のミッションクリティカル向け方針
  ([`open-web-server`](https://github.com/aon-co-jp/open-web-server)の
  4層防御通信)を踏み台・サーバー双方に踏襲する。エンドツーエンド暗号化を
  必須とする。
- 認証・課金・データ保全の考え方は今後 `PORTING.md` / `CLAUDE.md` で詳細化する。

## セキュリティ/プライバシー方針(端末情報の非漏洩)

**P2Pモード・サーバーモードいずれでも、相手や外部から自分の端末の真のIP
アドレスが見えない**ことを設計上の必須要件とする(二段階踏み台を必ず経由)。

- **既定はP2P、サーバーモードは利用者の選択制**: 自分の端末にサーバーを
  立てないP2Pモードを既定とし、ローカルの機器・データを外部に晒すリスクを
  避ける。サーバーモードへの切替は、利用者が明示的に選択した場合のみ
  (P2P接続がうまくいかない環境での代替、または上級者向け)。
- **二段階踏み台は常時経由**: 入口(Hop1)・出口(Hop2)を別ノードに分離し、
  単一ノードの侵害・ログだけでは「誰が・誰と話しているか」を特定できない
  構成にする(完全なTor相当の匿名性までは狙わないが、単純な1段中継より
  強い分離を確保する設計思想)。
- **シグナリングは最小情報のみ**: 呼び出し・接続確立に必要な最小限の情報
  (セッションID・公開鍵等)のみを扱い、端末識別子・OSバージョン・ファイル
  パス等の不要な端末固有情報は送信しない。
- **踏み台側ログの最小化**: 各HopのログにIPアドレス・端末識別子等を平文で
  残さない(必要な場合はマスキング・保持期間の限定を行う)。通話の音声/映像・
  翻訳結果の内容自体は中継後に保存しない(録画機能を提供する場合は利用者の
  明示的な選択によるオプトイン機能として別途設計)。
- **テレメトリ既定オフ**: 利用状況・クラッシュレポート等の送信はデフォルトで
  無効化し、送信する場合は利用者の明示的なオプトインを必須とする。
- **通信は暗号化必須**: クライアント⇔Hop1⇔Hop2⇔相手(またはサーバー)の
  各区間すべてでTLS/暗号化を必須とし、平文通信を許可しない。
- **相手への情報開示の最小化**: 通話相手に開示する情報(表示名等)は利用者が
  選択できるようにし、意図しない端末情報(実名・詳細な位置情報等)が自動的に
  相手へ渡らないようにする。
- **悪用対策の考慮(2026-09-27追記)**: IPを秘匿する仕組みは、利用者保護と
  同時に悪用(嫌がらせ・違法行為の身元隠し等)のリスクも伴うため、通報・
  ブロック機能や、法的要請への対応方針(ログ保持期間・開示条件)を実装着手時に
  別途検討する。

具体的な実装(二段階踏み台のノード配置・鍵交換方式・Hop間のプロトコル等)は、
`open-LiveKit`側の設計([`open-LiveKit`のPORTING.md](https://github.com/aon-co-jp/open-LiveKit/blob/main/PORTING.md)
参照)と整合させながら実装着手時に詳細化する。

## 音声翻訳エンジンの選定(調査完了・2026-09-26)

商用配布(easy-web.tokyoでの無償/有償配布)を前提に、ASR(音声認識)は
[Whisper](https://github.com/openai/whisper)(MIT、約99言語)、MT(機械翻訳)は
[MADLAD-400](https://huggingface.co/google/madlad400-3b-mt)(Apache-2.0、400+言語)を
軸とする構成を第一候補とした。Meta製のSeamlessM4T/SeamlessStreaming/MMSは言語数・
性能で優位だがCC-BY-NC 4.0(非商用限定)のため製品には採用しない。TTSは
[Piper](https://github.com/OHF-Voice/piper1-gpl)を候補としつつ、声モデルごとに
ライセンスが異なるため個別確認が必要。詳細な比較表・方針は
[`PORTING.md`](PORTING.md)「1. 音声翻訳エンジン調査結果」を参照。

## マイク音声のノイズ低減・音質向上・音声超解像(調査完了・2026-09-27)

Skype風の通話品質を実現するため、翻訳エンジン(ASR/MT/TTS)の前段で
マイク音声を前処理する構成を検討した。

| 技術 | 用途 | 候補 | ライセンス | 備考 |
|---|---|---|---|---|
| ノイズ抑制 | 背景雑音の除去 | [DeepFilterNet](https://github.com/Rikorose/DeepFilterNet) | MIT/Apache-2.0(モデルはBSD) | **コアDSPが元々Rust実装**、高品質・約40msの遅延で通話にも実用的。軽量な[RNNoise](https://github.com/xiph/rnnoise)(BSD)も代替候補 |
| 音声超解像(帯域拡張) | 狭帯域音声を広帯域化(画像の超解像のアナロジー) | [AudioSR](https://github.com/haoheliu/versatile_audio_super_resolution)、[VoiceFixer](https://github.com/haoheliu/voicefixer) | AudioSR: コードMIT・重みApache-2.0 | **既存の[`open-audio-sr`](https://github.com/aon-co-jp/open-audio-sr)で実装・実機検証済み**(AudioSRを採用し、単純補間との定量比較で高域エネルギー生成を確認済み) |
| 最新研究(2025〜2026) | よりリアルタイム向けの音声超解像 | StreamWSR(ストリーミング対応・軽量)、Wave-U-Mamba(ICASSP 2025) | 論文ベース、実装は要調査 | AudioSRは拡散モデルで処理が重く、**ライブ通話のリアルタイム制約には向かない可能性がある**ため、将来的な軽量化・高速化の候補として記録 |

**方針**: ノイズ抑制はDeepFilterNet(Rustコアとの親和性が高く、実装言語の
統一という既存方針に合致)を第一候補とする。音声超解像は、既に実装・実機
検証済みの`open-audio-sr`(AudioSR)を再利用する構想だが、**AudioSRは拡散
モデルでありリアルタイム通話向けではない可能性が高い**ため、ライブ通話の
音声パイプラインに組み込めるか(または録音後処理限定にするか)は実装着手時に
性能検証が必要(下記「関連リポジトリとの連携」参照)。

## 関連リポジトリとの連携(2026-09-27方針、構想段階)

音声処理(ノイズ抑制・音声超解像)と音声翻訳のパイプライン全体を高速化・
高品質化するため、以下のaon-co-jpエコシステム内リポジトリと連携する構想
がある(いずれも各リポジトリ側の成熟を待って実際の連携実装に着手する)。

- **[`open-audio-sr`](https://github.com/aon-co-jp/open-audio-sr)**:
  AudioSRによる音声超解像を実装・実機検証済みの既存資産。上記の通り
  リアルタイム性の検証が必要。
- **[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm)**: 音声認識
  (Whisper)の誤認識をLLMで文脈的に補正する、または翻訳の自然さをAIで
  向上させる用途を検討。
- **[`open-cuda`](https://github.com/aon-co-jp/open-cuda)**: ノイズ抑制・
  音声超解像・ASR/MT/TTS推論のGPU高速化(拡散モデルベースのAudioSRを
  リアルタイムに近づけるには特に重要)。
- **[`open-directx`](https://github.com/aon-co-jp/open-directx)**:
  `open-cuda`と共通の計算基盤を共有しつつ、クライアントの通話UI
  (字幕表示・音質メーター等)の高速描画に使う。

**現状の制約(`aruaru-vpn`と同様)**: `open-cuda`・`open-directx`は実体が
乏しく、連携実装は各リポジトリの成熟を待つ必要がある。`open-audio-sr`は
実装済みだが、TV CHATのライブ通話への統合(リアルタイム性の検証)は
別途必要。

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

---

作者 / Author: 石 塚 正 浩（MASAHIRO ISHIZUKA） (https://github.com/aon-co-jp)

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**
