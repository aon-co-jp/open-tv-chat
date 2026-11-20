# PORTING (open-tv-chat)

## 現状(2026-09-26)

設計ドキュメント段階。リポジトリ雛形(README.md/CLAUDE.md/PORTING.md)のみ整備。
コード実装は0行。

## 未確定事項(次回セッションで詰める)

1. ~~音声翻訳エンジンの選定~~ → **2026-09-26 調査完了**、下記「1. 音声翻訳エンジン
   調査結果」参照。
2. ~~プラットフォーム別クライアント実装方式~~ → **2026-09-26 方針案作成**、下記
   「2. クライアント実装方式」参照。
3. ~~easy-web.tokyo側の紹介ページ・ダウンロード導線~~ → **2026-09-26 方針案作成**、
   下記「3. easy-web.tokyo配布導線」参照。
4. ~~対応言語一覧(約130ヶ国語)の正式リスト~~ → **2026-09-26 サンプル30件作成**、
   下記「4. 対応言語リスト」参照(残りはISO 639-1/639-3ベースで実装時に生成)。

## 1. 音声翻訳エンジン調査結果(2026-09-26)

ASR(音声認識)/MT(機械翻訳)/TTS(音声合成)のカスケード構成(音声→テキスト→
翻訳テキスト→音声)を前提に、商用配布(easy-web.tokyoでの無償/有償配布)に耐える
ライセンスかどうかを軸に主要OSSを比較した。

| 段階 | モデル | 提供元 | 言語数 | ライセンス | 商用利用 |
|---|---|---|---|---|---|
| ASR | Whisper (large-v3) | OpenAI | 約99言語 | MIT | ✅ 可 |
| MT | MADLAD-400 (3B/7B/10B-mt) | Google Research | 400+言語 | Apache-2.0 | ✅ 可 |
| MT (代替) | NLLB-200 | Meta | 200言語 | CC-BY-NC 4.0 | ❌ 非商用のみ |
| ASR+MT+TTS一体 | SeamlessM4T v2 / SeamlessStreaming | Meta | 96言語(ASR)/76言語(音声対訳データ) | CC-BY-NC 4.0 | ❌ 非商用のみ |
| ASR+TTS一体 | MMS (Massively Multilingual Speech) | Meta | 1,100+言語 | CC-BY-NC 4.0 | ❌ 非商用のみ |
| TTS | Piper | Rhasspy/Open Home Foundation | 音声56言語/地域(エンジン自体はOSS) | エンジン: MIT〜GPL-3.0(フォーク後) / 声モデル: 声ごとに個別ライセンス | 声モデルは個別確認要 |
| TTS (代替) | Coqui XTTS v2 | Coqui/派生コミュニティ | 十数言語 | CPML(非商用寄り) | ⚠️ 要個別確認 |

**方針案**: Meta系(SeamlessM4T/SeamlessStreaming/MMS)は言語数・リアルタイム性能で
最有力だが**CC-BY-NC 4.0のため商用不可**。easy-web.tokyoでの配布(将来的な有償化の
可能性を排除しない)を踏まえ、**ASRはWhisper(MIT)、MTはMADLAD-400(Apache-2.0)を
軸とした商用利用可能な構成を第一候補**とする。TTSは声モデル単位でライセンスが
異なるPiperを使いつつ、各言語の同梱ボイスはMODEL_CARDを1件ずつ確認して選定する
(推測でライセンスを判断しない)。Meta系モデルは、将来的に非商用版・研究用途・
自社サーバーでの検証目的でのみ併用を検討する。

最終的な同時通訳(2〜10ヶ国語)のレイテンシ・精度検証は、実装フェーズでの
ベンチマーク作業として別途行う(本調査は「候補の絞り込み」までが範囲)。

## 2. クライアント実装方式(方針案・2026-09-26)

- Windows/macOS/Linuxの3デスクトップOSは、単一コードベースで賄うため
  **Rust + Tauri**を第一候補とする(`RPoem`/`open-web-server`と同じRustスタックで
  統一でき、`open-easy-web`のRust実装資産とも親和性が高い)。
- Android/iPhone(iOS)は、Tauri Mobileの対応状況(2026年時点で活発に開発中だが
  デスクトップ版ほど枯れていない)を実装着手時に再評価し、枯れていなければ
  各OSネイティブ実装(Kotlin/Swift)を別途検討する。判断は次回実装着手時に行う。
- 音声/映像のリアルタイム通信層(WebRTC相当)の実装方式(自前実装 or 既存OSS
  ライブラリのRustバインディング利用)は未確定、実装着手時に別途調査する。

## 3. easy-web.tokyo配布導線(方針案・2026-09-26)

- 紹介ページは`open-english`のWindows/Android版インストーラー・アンインストーラー・
  バージョン管理の運用を踏襲し、OS判定による推奨ダウンロードボタンの出し分けを
  行う(既存資産の再利用、実装コードの複製ではなくパターンの参照)。
  ※`open-english`側の実装詳細は本リポジトリへ複製しない(既存の索引方針どおり)。
- バージョン管理・リリースの発火条件(タグpush等)は`open-english`のリリースCI方式
  (v*タグpushで発火)を参考に、実装着手時に本リポジトリのCIとして構築する。
- 掲載場所(easy-web.tokyo内のURL/ナビゲーション上の位置)は、実装着手時に
  ユーザーへ確認してから配置する(新規配置先の相談ルールに準じる)。

## 4. 対応言語リスト(サンプル・2026-09-26作成、要ユーザー検証)

「英語名 (現地呼称) = ネイティブ表記 (英語での言語名)」形式のサンプル30件。
**ネイティブ表記はAIによる下書きであり、誤りが無いか実装着手時にネイティブ話者
または信頼できる一次情報での検証が必要**(このまま製品UIへ転記しない)。

| 国・地域名 (現地呼称) | ネイティブ表記 (言語の英語名) |
|---|---|
| Japan (Nihon) | 日本語 (Japanese) |
| Iran (Persia) | فارسی (Farsi) |
| China (Zhōngguó) | 中文 (Chinese, Mandarin) |
| South Korea (Hanguk) | 한국어 (Korean) |
| France | Français (French) |
| Germany (Deutschland) | Deutsch (German) |
| Spain (España) | Español (Spanish) |
| Italy (Italia) | Italiano (Italian) |
| Portugal | Português (Portuguese) |
| Russia (Rossiya) | Русский (Russian) |
| Saudi Arabia | العربية (Arabic) |
| India (Hindi圏) | हिन्दी (Hindi) |
| Thailand (Prathet Thai) | ภาษาไทย (Thai) |
| Vietnam (Việt Nam) | Tiếng Việt (Vietnamese) |
| Indonesia | Bahasa Indonesia (Indonesian) |
| Turkey (Türkiye) | Türkçe (Turkish) |
| Israel | עברית (Hebrew) |
| Greece (Ellada) | Ελληνικά (Greek) |
| Poland (Polska) | Polski (Polish) |
| Netherlands (Nederland) | Nederlands (Dutch) |
| Sweden (Sverige) | Svenska (Swedish) |
| Ukraine (Ukraina) | Українська (Ukrainian) |
| Myanmar | မြန်မာဘာသာ (Burmese) |
| Cambodia (Kampuchea) | ភាសាខ្មែរ (Khmer) |
| Mongolia | Монгол хэл (Mongolian) |
| Nepal | नेपाली (Nepali) |
| Sri Lanka | සිංහල (Sinhala) |
| Pakistan (Urdu圏) | اردو (Urdu) |
| Bangladesh | বাংলা (Bengali) |
| Finland (Suomi) | Suomi (Finnish) |

残り約100件は、上記と同じ形式でISO 639-1/639-3の全言語コードを基に実装時に
機械的に候補生成し、1件ずつネイティブ表記を検証しながら追加していく。

## 5. アーキテクチャ方針の変遷(2026-09-26、2回の指示変更を反映)

- **1回目の指示**(当初): 「利用者各自の端末で`open-web-server`を自動インストール
  し、自分の端末上に自動でWEBサーバーを立てる」構成として、インストーラー方式
  (自動セットアップ)・セキュリティ方針(ローカルのみバインド等)をREADME.mdに
  記載した。
- **2回目の指示**(修正): 「利用者どうしが互いの端末にWEBサーバーを立てないなら
  P2Pでも良い」「サーバーを立てるのとP2Pのどちらが良いかは相談」との指示を受け、
  P2P案とサーバー中継案のトレードオフ(利用者負担・端末情報の露出・翻訳処理の
  負荷分散等)をユーザーに提示。
- **最終決定**: 「WEBサーバーを立てる事とピアツーピアのハイブリッドで良い所どり。
  相手にIPアドレスが見えないセキュリティにしたい」との回答を受け、
  **既定モード=運営サーバー経由のリレー(相手にIP非開示・翻訳もサーバー側で
  一括処理)、オプションモード=利用者の明示的同意がある場合のみの直接P2P**、
  というハイブリッド方式に確定した。利用者が各自の端末に`open-web-server`を
  インストールする構成は撤回し、`open-web-server`は運営(aon-co-jp)側の
  リレー/シグナリングサーバーとして使う構成に変更した。

詳細は[`README.md`](README.md)「アーキテクチャ概要」「セキュリティ/プライバシー
方針」を参照(2026-09-27に下記8の通りさらに変更されているため、この節は
歴史的経緯として残す)。

## 8. 接続モード方針の再変更(2026-09-27、3回目の指示変更)

ユーザーから「基本はWEBサーバーを立ててTV CHATしてもらうが、サーバーが
立たない環境や端末では自動でP2Pになり、さらに二段階の踏み台+フリーVPN
アプリも同時開発して、元のIPアドレスがばれずに安全にTV CHATできる工夫を
一緒に検討したい」との提案を受け、以下を協議・決定した。

- **IP秘匿の実現方式**: 新規の汎用VPNアプリ(選択肢C)は、130ヶ国展開という
  目標と衝突する法的リスク(VPN事業は国により合法性が大きく異なる。中国・
  ロシア・UAE等は規制/禁止)、世界規模の出口ノード運用コスト、悪用対策等、
  TV CHATとは別次元の大きな事業になるため**切り離し**、
  **「既存のSFUリレー(`open-LiveKit`)を拡張する」方式(選択肢A)を採用**
  することで合意した。`open-LiveKit`は元々「相手にIPを見せない」設計を
  持っていたため、これをサーバーモード・P2Pモードの両方で経由する
  「二段階踏み台(Hop1/Hop2に分離した中継)」として拡張する。
- **接続モード(4回目の指示変更、同日中に再修正)**: 「サーバーを立てても
  IPは踏み台で隠せるとしても、そもそも自分の端末にサーバーを立てない方が
  ローカルの機器・DATAへのリスクが小さく、より安全性が高い」との指摘を
  受け、**既定=P2Pモード、`open-web-server`を自分の端末で立てる
  サーバーモードはオプション(P2Pが繋がりにくい環境向けの代替、または
  上級者向け)**に確定した。
- P2Pモード・サーバーモードいずれの場合も、二段階踏み台を経由させることで
  「相手や外部から自分の端末の真のIPアドレスが見えない」を両モード共通の
  必須要件として維持する。

詳細な最新のアーキテクチャ図・セキュリティ方針は[`README.md`](README.md)
「アーキテクチャ概要」「セキュリティ/プライバシー方針」(2026-09-27版、
P2P既定)を参照。

## 9. マイク音声処理・関連リポジトリ連携(2026-09-27)

ノイズ抑制はDeepFilterNet(Rustコア、MIT/Apache-2.0)、音声超解像は既存の
`open-audio-sr`(AudioSR)を再利用する方針を決定。ただしAudioSRは拡散
モデルでライブ通話のリアルタイム制約に向かない可能性があり、実装着手時に
性能検証が必要。`aruaru-llm`(ASR誤認識補正等)・`open-cuda`(GPU高速化)・
`open-directx`(UI高速描画)との連携も構想(`open-cuda`/`open-directx`は
実体が乏しく連携は各リポジトリの成熟待ち)。詳細は[`README.md`](README.md)
「マイク音声のノイズ低減・音質向上・音声超解像」「関連リポジトリとの連携」
を参照。

## 次回再開ポイント

- 上記4のネイティブ表記の検証(専門家・一次資料での裏取り)
- Whisper + MADLAD-400 + Piperの実機プロトタイプ(2ヶ国語間最小構成)
- 二段階踏み台(Hop1/Hop2)の技術設計・実装は`open-LiveKit`側で行う
  (詳細は[`open-LiveKit`のPORTING.md](https://github.com/aon-co-jp/open-LiveKit/blob/main/PORTING.md)
  「次回再開ポイント」を参照。webrtc-rs疎通確認・`aruaru-db`ルーム状態
  スキーマ・翻訳エンジンAgentフック接続・RPoemゲートウェイ実配線の順で
  進行中)
- クライアント側の「サーバーモードへの切替UI」設計(利用者が明示的に
  選択する導線、P2P接続失敗時の案内文言等)は未着手
- Tauriでのデスクトップクライアント雛形作成

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
