use std::io;

use std::io::BufWriter;
use std::io::Write;

use std::io::BufRead;

use serde_json::Map;
use serde_json::Value;

use aho_corasick::AhoCorasick;
use aho_corasick::MatchKind;

pub fn line2map2ids2map2writer<W>(
    line: &[u8],
    body_key: &str,
    tags_key: &str,
    patterns: &AhoCorasick,
    writer: &mut W,
    buf: &mut Vec<Value>,
) -> Result<(), io::Error>
where
    W: Write,
{
    let parsed: Value = serde_json::from_slice(line).map_err(io::Error::other)?;
    let mut m: Map<String, Value> = match parsed {
        Value::Object(m) => Ok(m),
        _ => Err(io::Error::other("invalid object")),
    }?;
    let obody: Option<&Value> = m.get(body_key);
    match obody {
        None => {
            writer.write_all(line)?;
            writeln!(writer)?;
            Ok(())
        }
        Some(body) => {
            let msg: &str = match body {
                Value::String(s) => Ok(s),
                _ => Err(io::Error::other("invalid body")),
            }?;
            let founds = patterns.try_find_iter(msg).map_err(io::Error::other)?;
            let ids = founds.map(|m| m.pattern());

            buf.clear();
            for id in ids {
                let u: u32 = id.as_u32();
                let v: Value = u.into();
                buf.push(v);
            }
            m.insert(tags_key.into(), Value::from(buf.clone()));
            serde_json::to_writer(writer.by_ref(), &m).map_err(io::Error::other)?;
            writeln!(writer)?;
            Ok(())
        }
    }
}

pub fn lines2ids2writer<I, W>(
    lines: I,
    body_key: &str,
    tags_key: &str,
    patterns: &AhoCorasick,
    writer: &mut W,
    buf: &mut Vec<Value>,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        line2map2ids2map2writer(&line, body_key, tags_key, patterns, writer, buf)?;
    }
    writer.flush()
}

pub struct Config {
    pub body_key: String,
    pub tags_key: String,
}

pub const BODY_KEY_DEFAULT: &str = "body";
pub const TAGS_KEY_DEFAULT: &str = "tags";

pub const MATCH_KIND_DEFAULT: &str = "standard";

impl Default for Config {
    fn default() -> Self {
        Self {
            body_key: BODY_KEY_DEFAULT.into(),
            tags_key: TAGS_KEY_DEFAULT.into(),
        }
    }
}

pub fn str2match_kind(s: &str) -> MatchKind {
    match s {
        "long" => MatchKind::LeftmostLongest,
        "longest" => MatchKind::LeftmostLongest,
        "most-longest" => MatchKind::LeftmostLongest,
        "mostlongest" => MatchKind::LeftmostLongest,
        "left-most-longest" => MatchKind::LeftmostLongest,
        "leftmostlongest" => MatchKind::LeftmostLongest,
        "1st" => MatchKind::LeftmostFirst,
        "first" => MatchKind::LeftmostFirst,
        "mostfirst" => MatchKind::LeftmostFirst,
        "most-first" => MatchKind::LeftmostFirst,
        "left-most-first" => MatchKind::LeftmostFirst,
        "leftmostfirst" => MatchKind::LeftmostFirst,
        "standard" => MatchKind::Standard,
        "std" => MatchKind::Standard,
        _ => MatchKind::Standard,
    }
}

impl Config {
    pub fn stdin2ids2maps2stdout<S, P>(&self, patterns: P, kind: &str) -> Result<(), io::Error>
    where
        P: Iterator<Item = S>,
        S: AsRef<[u8]>,
    {
        let typ: MatchKind = str2match_kind(kind);
        let pat: AhoCorasick = AhoCorasick::builder()
            .match_kind(typ)
            .build(patterns)
            .map_err(io::Error::other)?;

        let i = io::stdin();
        let il = i.lock();
        let lines = il.split(b'\n');

        let o = io::stdout();
        let mut ol = o.lock();

        let mut buf: Vec<Value> = vec![];

        let mut bw = BufWriter::new(&mut ol);
        lines2ids2writer(
            lines,
            &self.body_key,
            &self.tags_key,
            &pat,
            &mut bw,
            &mut buf,
        )?;
        bw.flush()?;

        drop(bw);
        ol.flush()
    }
}
