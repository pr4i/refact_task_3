pub struct TrendLimit(i64);

impl TrendLimit {
    pub fn new(v: Option<u32>) -> Self {
        let raw = v.unwrap_or(30) as i64;
        // clamp 2..200
        let clamped = raw.max(2).min(200);
        Self(clamped)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}
