pub struct TimeSeriesDB { path: String }

impl TimeSeriesDB {
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { path: path.to_string() })
    }
    
    pub fn write(&self, _metric: &str, _value: f64, _ts: u64) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    pub fn query(&self, _metric: &str, _start: u64, _end: u64) -> Result<Vec<(u64, f64)>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}
