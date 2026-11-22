# 常駐翻訳(MADLAD-400 3B q4k)の再現手順 / Resident translation, how to reproduce

日本語: candle の `quantized-t5` 例に、標準入力から1行ずつ訳す常駐ループを足したパッチです(`madlad-resident-candle.patch`、1行目は基準の candle コミット)。
1. `git clone https://github.com/huggingface/candle` し、1行目のコミットに合わせて `git apply` する。
2. `RUSTFLAGS="-C target-cpu=native" cargo build --release --example quantized-t5`(WSL、手元PCで)。
3. 環境変数 `MT_TOKENIZER` にトークナイザーのパスを指定して起動し、1行に1文ずつ入力する。モデルは `F:\_work\models\madlad400-3b-mt-q4k`(リポジトリには入れない)。
実測: 読み込み約30秒は最初の1回だけ。短い文ごとだと1文あたり約3〜8秒(約2.7語/秒)。まだリアルタイムには届かない。

English: A patch that adds a resident loop (one line per sentence from stdin) to candle's `quantized-t5` example. The first line of the patch file is the base candle commit. Apply it, build with `RUSTFLAGS="-C target-cpu=native"`, set `MT_TOKENIZER`, and feed one sentence per line. Measured: the 30 s load happens once; about 3 to 8 s per short sentence (about 2.7 tokens/s). Still not real time.
