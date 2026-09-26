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

## 次回再開ポイント

技術調査・方針案は一通り出揃った。次回は以下いずれかから着手:
- 上記4のネイティブ表記の検証(専門家・一次資料での裏取り)
- Whisper + MADLAD-400 + Piperの実機プロトタイプ(2ヶ国語間の最小構成)
- Tauriでのデスクトップクライアント雛形作成
