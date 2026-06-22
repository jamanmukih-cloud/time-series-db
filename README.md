# Time-Series DB 📈

Embedded time-series database with compression.

## Features

- **Compression**: Gorilla + Delta encoding
- **Downsampling**: Automatic rollups
- **Continuous Aggregation**: Pre-computed queries
- **Retention Policies**: Auto-cleanup

## Performance

| Metric | Value |
|--------|-------|
| Write | 1M points/s |
| Query | 500K points/s |
| Compression | 10:1 ratio |

## Quick Start

```rust
let db = TimeSeriesDB::open("/data/ts")?;
db.write("cpu.usage", 95.5, timestamp::now())?;
let points = db.query("cpu.usage", start, end)?;
```

## License

MIT