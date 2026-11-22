//! vad-chunker <16kHz mono 16bit WAV> <出力フォルダ> [最小区間ms] [最小無音ms]: 無音で区切り、各区間を f32 LE(`chunk_N.f32`)で書き、
//! 区間の「開始秒 終了秒」を標準出力へ出す。rs-whisper-transcribe にそのまま渡せる。
//! 試作: 区切りは単純なエネルギー(RMS)方式。雑音の多い環境では未確認。

pub const RATE: usize = 16_000;

#[derive(Clone, Debug)]
pub struct Params {
    pub frame_ms: usize,
    pub min_silence_ms: usize, // これ以上続く無音で区切る
    pub min_len_ms: usize,     // これより短い区間は区切らない
    pub max_len_ms: usize,     // これを超えたら無音が無くても最小エネルギー点で切る
    pub overlap_ms: usize,     // 次の区間を前の終端よりこれだけ早く始める
    pub rel_threshold: f32,    // 最大RMSに対する無音しきい値(割合)
}

impl Default for Params {
    fn default() -> Self {
        Params { frame_ms: 20, min_silence_ms: 300, min_len_ms: 1500, max_len_ms: 8000, overlap_ms: 300, rel_threshold: 0.05 }
    }
}

fn frame_rms(s: &[f32], fl: usize) -> Vec<f32> {
    s.chunks(fl).map(|c| (c.iter().map(|x| x * x).sum::<f32>() / c.len() as f32).sqrt()).collect()
}

/// 返り値は (開始サンプル, 終了サンプル) の列。オーバーラップ込み。
pub fn segment(s: &[f32], p: &Params) -> Vec<(usize, usize)> {
    let fl = RATE * p.frame_ms / 1000;
    let rms = frame_rms(s, fl);
    let peak = rms.iter().cloned().fold(0.0, f32::max);
    if peak <= 0.0 {
        return vec![];
    }
    let thr = peak * p.rel_threshold;
    let min_sil = p.min_silence_ms / p.frame_ms;
    let min_len = p.min_len_ms / p.frame_ms;
    let max_len = p.max_len_ms / p.frame_ms;
    let ov = (p.overlap_ms / p.frame_ms) * fl;
    let mut cuts: Vec<usize> = vec![]; // 区切りのフレーム位置
    let mut start = 0usize;
    let mut sil = 0usize;
    for i in 0..rms.len() {
        sil = if rms[i] < thr { sil + 1 } else { 0 };
        let len = i + 1 - start;
        if sil >= min_sil && len >= min_len + min_sil {
            cuts.push(i + 1 - sil / 2); // 無音の真ん中で切る
            start = i + 1 - sil / 2;
            sil = 0;
        } else if len >= max_len {
            let lo = i + 1 - max_len / 4;
            let (mut best, mut bv) = (i, f32::MAX);
            for j in lo..=i {
                if rms[j] < bv { bv = rms[j]; best = j; }
            }
            cuts.push(best);
            start = best;
            sil = 0;
        }
    }
    let mut out = vec![];
    let mut a = 0usize;
    for c in cuts.into_iter().chain(std::iter::once(rms.len())) {
        let end = (c * fl).min(s.len());
        let begin = if a == 0 { 0 } else { (a * fl).saturating_sub(ov) };
        if end > begin {
            out.push((begin, end));
        }
        a = c;
    }
    out
}

pub fn read_wav16(b: &[u8]) -> Result<Vec<f32>, String> {
    if b.len() < 44 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a WAV".into());
    }
    let mut i = 12;
    let (mut ch, mut rate, mut bits) = (0u16, 0u32, 0u16);
    while i + 8 <= b.len() {
        let id = &b[i..i + 4];
        let sz = u32::from_le_bytes([b[i + 4], b[i + 5], b[i + 6], b[i + 7]]) as usize;
        let body = i + 8;
        if id == b"fmt " {
            ch = u16::from_le_bytes([b[body + 2], b[body + 3]]);
            rate = u32::from_le_bytes([b[body + 4], b[body + 5], b[body + 6], b[body + 7]]);
            bits = u16::from_le_bytes([b[body + 14], b[body + 15]]);
        } else if id == b"data" {
            if ch != 1 || rate as usize != RATE || bits != 16 {
                return Err(format!("need 16kHz mono 16bit (got {ch}ch {rate}Hz {bits}bit)"));
            }
            let end = (body + sz).min(b.len());
            return Ok(b[body..end].chunks_exact(2).map(|x| i16::from_le_bytes([x[0], x[1]]) as f32 / 32768.0).collect());
        }
        i = body + sz + (sz & 1);
    }
    Err("no data chunk".into())
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: vad-chunker <in.wav 16k mono 16bit> <outdir>");
        std::process::exit(2);
    }
    let s = read_wav16(&std::fs::read(&a[1]).expect("read wav")).expect("wav");
    std::fs::create_dir_all(&a[2]).expect("outdir");
    let mut p = Params::default();
    if let Some(v) = a.get(3).and_then(|x| x.parse().ok()) { p.min_len_ms = v; }
    if let Some(v) = a.get(4).and_then(|x| x.parse().ok()) { p.min_silence_ms = v; }
    for (n, (b, e)) in segment(&s, &p).into_iter().enumerate() {
        let bytes: Vec<u8> = s[b..e].iter().flat_map(|x| x.to_le_bytes()).collect();
        std::fs::write(format!("{}/chunk_{n}.f32", a[2]), bytes).expect("write");
        println!("{n} {:.2} {:.2}", b as f32 / RATE as f32, e as f32 / RATE as f32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(sec: f32) -> Vec<f32> {
        (0..(sec * RATE as f32) as usize).map(|i| (i as f32 * 0.1).sin() * 0.5).collect()
    }
    fn quiet(sec: f32) -> Vec<f32> {
        vec![0.0; (sec * RATE as f32) as usize]
    }

    #[test]
    fn splits_at_silence_with_overlap() {
        let mut s = tone(3.0);
        s.extend(quiet(0.6));
        s.extend(tone(3.0));
        let seg = segment(&s, &Params::default());
        assert_eq!(seg.len(), 2);
        assert!(seg[1].0 < seg[0].1, "overlap expected");
        assert_eq!(seg[1].1, s.len());
    }

    #[test]
    fn short_pause_does_not_split() {
        let mut s = tone(1.0);
        s.extend(quiet(0.1));
        s.extend(tone(1.0));
        assert_eq!(segment(&s, &Params::default()).len(), 1);
    }

    #[test]
    fn long_speech_is_force_cut() {
        let s = tone(20.0);
        let seg = segment(&s, &Params::default());
        assert!(seg.len() >= 3);
        assert!(seg.iter().all(|(b, e)| e - b <= (Params::default().max_len_ms + 1000) * RATE / 1000));
    }

    #[test]
    fn silence_only_gives_nothing() {
        assert!(segment(&quiet(2.0), &Params::default()).is_empty());
    }

    #[test]
    fn wav_rejects_wrong_format() {
        assert!(read_wav16(b"nope").is_err());
    }
}
