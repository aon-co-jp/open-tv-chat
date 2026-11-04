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
