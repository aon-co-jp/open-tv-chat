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
easy-web.tokyoから配布する。**既定はP2Pモード(自分の端末にサーバーを
立てない)**、`open-web-server`を自分の端末で立てる「サーバーモード」は
オプション(P2Pが繋がりにくい環境向けの代替、または上級者向け)
(2026-09-27決定、4回目の方針変更。詳細は[`PORTING.md`](PORTING.md)
「8. 接続モード方針の再変更」参照)。どちらの
モードでも[`open-LiveKit`](https://github.com/aon-co-jp/open-LiveKit)の
SFUリレーを「二段階踏み台(Hop1/Hop2に分離)」として経由させ、相手や外部に
真のIPアドレスを見せない。詳細な構想は[`README.md`](README.md)を参照。

## 技術方針

- サーバーサイド(二段階踏み台/シグナリング): Rust +
  [`open-LiveKit`](https://github.com/aon-co-jp/open-LiveKit)
  (LiveKit参考のRust製SFU) + [`RPoem`](https://github.com/aon-co-jp/RPoem)
  (WunderGraph Cosmo互換、REST API不要、Tomcat互換)。
- Webサーバー(利用者端末側、オプションのサーバーモード): [`open-web-server`](https://github.com/aon-co-jp/open-web-server)
  (Apache/Nginx互換)。既定はP2Pモードで、サーバーモードは利用者が
  明示的に選択した場合のみ使う。
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

## セキュリティ/プライバシー方針(必須要件)

**P2Pモード・サーバーモードいずれでも、相手や外部から自分の端末の真のIP
アドレスが見えないこと**を設計上の必須要件とする(2026-09-27決定: 新規の
汎用VPNアプリは切り離し、既存の`open-LiveKit`(SFUリレー)を「二段階踏み台
(Hop1/Hop2に分離)」として拡張する選択肢Aを採用)。既定はP2Pモードとし、
自分の端末にサーバーを立てないことでローカルの機器・DATAへのリスクを
避ける(サーバーを立ててもIPは踏み台で隠せるが、それでもP2Pの方が安全性が
高いというユーザー判断による)。シグナリング
最小情報化・テレメトリ既定オフ・通信暗号化必須・踏み台側ログの端末情報
マスキング・悪用対策(通報/ブロック機能、法的要請対応方針)も必須要件。
詳細は[`README.md`](README.md)「セキュリティ/プライバシー方針」節を参照。
実装時はこの方針への適合を必ず確認すること。

## HANDOFF

- **2026-09-26 リポジトリ新設**: `aon-co-jp/open-tv-chat`を新規作成し、
  設計ドキュメント(README.md/CLAUDE.md/PORTING.md)のみを整備した段階。
  実装(クライアント本体・サーバーサイド・音声翻訳エンジン)は未着手。
- **2026-09-26 4項目の技術調査・方針案**: (1)音声翻訳エンジン=ASR:Whisper(MIT)
  +MT:MADLAD-400(Apache-2.0)を商用可能な第一候補として選定(Meta製Seamless系/MMS
  はCC-BY-NC 4.0で非商用のため不採用)、TTSはPiper候補(声モデル毎に要ライセンス
  個別確認)。(2)クライアントはRust+Tauri候補(Windows/macOS/Linux)、Android/iPhone
  はTauri Mobileの成熟度を実装着手時に再評価。(3)easy-web.tokyo配布導線は
  `open-english`のインストーラー/リリースCI運用を参考にする方針。(4)対応言語
  サンプル30件(要ネイティブ話者検証)を作成。詳細は[`PORTING.md`](PORTING.md)
  「1〜4」の各節、次回再開ポイントも同ファイル末尾を参照。
- **2026-09-26 アーキテクチャ方針を2回改訂、ハイブリッド方式に確定**: 当初は
  「利用者各自の端末に`open-web-server`を自動インストールしてWEBサーバーを
  立てる」構成だったが、ユーザーから「互いの端末にサーバーを立てないならP2Pでも
  良い」「サーバーを立てるかP2Pかは相談」との指示を受けてトレードオフを提示、
  最終的に「**既定=運営サーバー経由のリレー(相手にIP非開示・翻訳もサーバー側で
  一括処理)、オプション=利用者の明示的同意がある場合のみの直接P2P**」という
  ハイブリッド方式に確定した。`open-web-server`は利用者端末ではなく運営
  (aon-co-jp)側のリレー/シグナリングサーバーとして使う。詳細な変遷は
  [`PORTING.md`](PORTING.md)「5. アーキテクチャ方針の変遷」を参照。
  具体的な実装(SFU/リレーサーバーの技術選定・P2Pモード切替UI等)は未着手。
- **2026-09-27 接続モード方針を再変更(3回目→同日中に4回目)**: ユーザーから
  「基本はWEBサーバーを立ててTV CHAT、立たない環境では自動P2P、さらに
  二段階踏み台+フリーVPNアプリで安全にできないか一緒に検討したい」との
  提案を受け協議。汎用VPNアプリ(選択肢C)は130ヶ国展開との法的リスク・
  運用コストの観点で切り離し、**既存の`open-LiveKit`(SFUリレー)を
  Hop1/Hop2に分離した「二段階踏み台」として拡張する方式(選択肢A)を採用**。
  接続モードは当初「既定=利用者自身の端末で`open-web-server`を自動起動、
  起動不可な環境ではP2Pにフォールバック」で合意しかけたが、同日中にさらに
  ユーザーから「サーバーを立ててもIPは踏み台で隠せるとしても、そもそも
  自分の端末にサーバーを立てない方が安全性が高い」との指摘があり、
  **最終的に既定=P2Pモード、サーバーモードはオプション(P2Pが繋がりにくい
  環境向けの代替、または上級者向け)に確定**した。どちらのモードでも
  二段階踏み台を経由し、相手・外部に真のIPアドレスを見せない。詳細は
  [`PORTING.md`](PORTING.md)「8. 接続モード方針の再変更」を参照。二段階
  踏み台の技術設計・実装は`open-LiveKit`側で進行中
  ([`open-LiveKit`のPORTING.md](https://github.com/aon-co-jp/open-LiveKit/blob/main/PORTING.md)
  参照)。

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

## 2026-11-22 追記: open-tv-chat の現状と次の一歩(日本語のみ)
- **現状**: 設計文書のみで、実装コードはまだありません。手元のクローンは `origin/main` に追従しました(`3668e72`)。
- **次の一歩(候補)**: 2言語間の翻訳パイプライン試作(音声認識→翻訳→音声合成)。音声認識は手元の `rs-whisper` と `ggml-tiny.bin` が使えます。翻訳の `MADLAD-400`(約3B)は手元に無く、ダウンロードの許可待ちです。許可が無い間は、手元の Qwen2.5(`aruaru-llm`)で形と遅延だけ先に測る案があります(品質は設計の目標に届かない見込み)。
- **決まりごと**: 未了は確認せず自動で進めますが、ダウンロードは許可を得てから行います。実機で確かめてから「できた」と書きます。

## 2026-11-22 追記: 音声認識→翻訳の通し計測(日本語のみ)
- **許可を得て MADLAD-400 3B(q4k、約1.65GB)をダウンロード**しました(`F:\_work\models\madlad400-3b-mt-q4k`、リポジトリには入れない)。上の「許可待ち」は解消済みです。
- **計測(CPU のみ・32GB・GPUなし、WSL)**: `rs-whisper`(tiny)で `jfk.wav`(11秒)を文字起こし→MADLAD で日本語へ翻訳。文字起こしは正しく(約10.5秒)、翻訳も自然でした(「アメリカ人同胞の皆さん、自分の国が自分のために何をできるかを尋ねるな。…」)。翻訳は 50語で約1.9語/秒、読み込み込みで約53秒、メモリ約4.8GB。合計で約1分: **リアルタイム会話には遅く、そのままでは設計の目標に届きません**。
- 英語→日本語の短文では前半の一文が落ちる例がありました(長い入力は文ごとに区切って訳す必要がありそうです)。
- **次の一手**: 常駐化して読み込みを1回にする、短い文ごとのストリーミング翻訳、小さい翻訳モデルや GPU/`open-cuda` の利用、音声合成(Piper/Kokoro)の追加。実行手順は candle の quantized-t5 例(環境変数 `MT_TOKENIZER`)を使用。

## 2026-11-22 追記: 翻訳の高速化の知見を関連リポジトリで共有 / Sharing the translation-speed findings (日本語・English)

### 日本語
**共有する相手(同時に開発する)**: `open-tv-chat`(音声翻訳の本体)、`maid-cafe-english`(`maid-cafe.tokyo` の英語学習。翻訳・音声)、`maid-cafe-tokyo`(`https://maid-cafe.tokyo` の運用先)、`aruaru-llm`・`open-cuda`・`open-cpu`・`open-directx`(推論の土台)。なお、`open-maid-cafe` という名前のリポジトリは `aon-co-jp` に**存在しません**(あるのは `maid-cafe-tokyo`・`maid-cafe-english`・`maid-cafe-se`・`maid-cafe-programming-school`)。
**実測(CPU のみ、Ryzen 9 3950X・WSL・GPU は GT 730 2GB で実質使えない)**: 翻訳モデル MADLAD-400 3B(q4k、約1.65GB)を candle の quantized-t5 例で実行。
- 読み込みに約30秒かかるため、**常駐させて1回だけ読み込む**(標準入力から1行ずつ訳す形に改造、環境変数 `MT_TOKIENIZER` でトークナイザーを指定)。
- **短い文ごとに訳す**と、1文あたり約3〜8秒(2.7語/秒)。長い文をまとめるより、前半が落ちにくい。
- `RUSTFLAGS="-C target-cpu=native"` で語速が約2.3→2.7語/秒(約15%向上)。
- 音声認識(`rs-whisper` tiny、11秒の音声で約10.5秒)と合わせても、**そのままではリアルタイムに届かない**。次の候補は、小さい翻訳モデル、`open-cuda-llm` への T5 追加と比較、`open-cuda-whisper` との音声認識の比較、音声合成(Piper/Kokoro)。
**取り込み方針**: maid-cafe-english の翻訳・音声(AI回答や聞き取り)は、常駐+文ごと+native ビルドをまず適用する。新しい REST は作らず Connect/gRPC で呼ぶ。VPS は4コアで遅いので、**測定・開発は手元PC**、VPS は配備と確認だけ。
**未了**: (1)`open-cuda-llm` への T5 実装と candle との比較、(2)`open-cuda-whisper` と `rs-whisper` の比較、(3)GPU(Vulkan/DirectX)の実測(この PC の GT 730 は古く効果は期待薄)、(4)各リポジトリのコードへの実際の反映(今回は文書のみ)。

### English
**Who shares this (developed together)**: `open-tv-chat` (the voice-translation app), `maid-cafe-english` (English learning for `maid-cafe.tokyo`: translation and voice), `maid-cafe-tokyo` (the site running at `https://maid-cafe.tokyo`), and `aruaru-llm`, `open-cuda`, `open-cpu`, `open-directx` (the inference base). Note: there is **no** repository named `open-maid-cafe` in `aon-co-jp` (the existing ones are `maid-cafe-tokyo`, `maid-cafe-english`, `maid-cafe-se` and `maid-cafe-programming-school`).
**Measured (CPU only; Ryzen 9 3950X, WSL; the GT 730 2 GB GPU is effectively unusable)**: the MADLAD-400 3B translation model (q4k, about 1.65 GB) run with candle's quantized-t5 example.
- Loading takes about 30 s, so **keep it resident and load once** (patched to translate one line at a time from stdin; the tokenizer path comes from env `MT_TOKENIZER`).
- **Translating short sentences one by one** takes about 3 to 8 s each (2.7 tokens/s) and drops the first half less often than long input.
- `RUSTFLAGS="-C target-cpu=native"` raised speed from about 2.3 to 2.7 tokens/s (about 15%).
- Even with speech recognition (`rs-whisper` tiny, about 10.5 s for 11 s of audio) it is **still not real time**. Next candidates: a smaller translation model, adding T5 to `open-cuda-llm` and comparing, comparing `open-cuda-whisper` for recognition, and speech synthesis (Piper/Kokoro).
**Adoption policy**: apply resident + sentence-by-sentence + native build first to maid-cafe-english's translation and voice (AI answers and listening). No new REST; call over Connect/gRPC. The VPS has 4 slow cores, so **measure and develop on the local PC**; use the VPS only to deploy and verify.
**Open**: (1) T5 in `open-cuda-llm` and a comparison with candle; (2) `open-cuda-whisper` vs `rs-whisper`; (3) GPU (Vulkan/DirectX) measurements (this PC's GT 730 is old, little gain expected); (4) real code changes in each repository (this round is documentation only).

### 再開用メッセージ(2026-11-22、翻訳高速化・関連リポジトリ共有) / Resume message
**日本語**: 前の続きです。`F:\runo\PORTING.md` 末尾の「翻訳の高速化の知見を関連リポジトリで共有」を読んでから、`open-tv-chat`・`maid-cafe-english`・`maid-cafe-tokyo`(`open-maid-cafe` という名前のリポジトリは存在しません)・`aruaru-llm`・`open-cuda`・`open-cpu`・`open-directx` の未了を、確認なしで自動で進めてください。順番: (1)常駐+文ごと翻訳を maid-cafe-english のコードへ反映、(2)`open-cuda-llm` に T5 を追加して candle と比較、(3)`open-cuda-whisper` と `rs-whisper` の比較、(4)GPU 実測。作業は手元PC(WSL)、VPS は配備と確認のみ。決まり: 本物で確認してから完了、秘密は書かない、新しいRESTは作らない、返答は日本語のみ。
**English**: Continuing from before. Read "Sharing the translation-speed findings" at the end of `F:\runo\PORTING.md`, then keep working through the open items of `open-tv-chat`, `maid-cafe-english`, `maid-cafe-tokyo` (there is no repository named `open-maid-cafe`), `aruaru-llm`, `open-cuda`, `open-cpu` and `open-directx` without asking. Order: (1) apply resident + sentence-by-sentence translation to maid-cafe-english's code; (2) add T5 to `open-cuda-llm` and compare with candle; (3) compare `open-cuda-whisper` with `rs-whisper`; (4) GPU measurements. Work on the local PC (WSL); use the VPS only to deploy and verify. Rules: verify on real systems before saying done, never write secrets, no new REST.

## Versionless API の方針 / Versionless API policy (2026-11-25)
**日本語**: このリポジトリのAPIは **Versionless**(URL・パッケージ名に `v1` 等を付けず、互換性を壊さず足すだけで進化させる)。欄は足すだけ・足す欄は省略可能・知らない欄は無視・名前変更は新名を足して旧を非推奨に(期限は既定で6か月以上、使用0を確認してから削除)・enumは未知の値を許容・安全上の緊急修正は例外。既存の `.v1` は呼び出し側を壊さないよう当面残し、版なしの別名を追加して段階的に畳む。詳細・出典は [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md) 末尾。適用はこれから順に行う。
**English**: This repository's APIs are **Versionless**: no `v1` in URLs or package names; evolve by adding only, without breaking compatibility. Only add fields, new fields are optional, ignore unknown fields, rename by adding the new name and deprecating the old (default deadline 6 months or more, remove only after confirming zero use), tolerate unknown enum values, emergency security fixes are the exception. Existing `.v1` names stay for now so callers do not break; a versionless alias is added and `.v1` is retired in stages. Details and sources: the end of [`runo/API-POLICY.md`](https://github.com/aon-co-jp/runo/blob/main/API-POLICY.md). Rollout is step by step.

## 完成まで作り切る方針 / Finish everything policy (2026-11-27)
**日本語**: 全リポジトリで、**未確認は自動で確認し、未実施は実施し、未了は完成させる**(確認を求めず進める)。使用頻度が低いと判断される機能や、**1から作り直す大規模開発になる内容でも、やめずに最後まで開発・TEST・DEBUGする**(別の方式で済ませない、後回しにしない)。進め方: 調査(英語・日本語の検索とGitHub) → 設計 → 実装 → 実機/実データで確認 → 失敗は原因を直して再試験 → README・CLAUDE・PORTING・POLICY を日英で更新 → push。「できた」と書くのは実測した範囲だけ、確認できなかった所は「未確認」と正直に書く。例外(確認を取る): 元に戻せない削除、秘密の公開、所有者の認証・決済・実機操作が要るもの、利用者のシステム設定の変更。
**English**: In every repository, **verify what is unverified, do what is not yet done, and finish what is unfinished** automatically, without asking. Even features that look rarely used, or work that means **a large rewrite from scratch, are developed, tested and debugged to the end** (no substituting another approach, no postponing). Method: research (English and Japanese search plus GitHub) -> design -> implement -> verify on real devices or data -> fix causes and retest on failure -> update README, CLAUDE, PORTING and POLICY in Japanese and English -> push. Write "done" only for what was measured, and say "unverified" honestly for the rest. Exceptions that still need confirmation: irreversible deletions, publishing secrets, anything needing the owner's authentication, payment or real-device actions, and changes to the user's system settings.