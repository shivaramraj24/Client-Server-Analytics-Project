# Client-Server Analytics — DS210 Project

A client-server analytics system written in Rust that runs structured queries over CSV datasets via RPC. The server loads a dataset on startup and exposes two RPCs — one that returns the full dataset, and one that runs a filter/group-by/aggregation query on it. The client sends queries and prints results.

Two datasets are included: student grades and metal album ratings (generated with a Python script using normal distributions per album).

---

## How it works

The server loads a CSV file at startup and holds it in memory for the lifetime of the process. Clients connect and send queries over RPC. A query has three parts:

- **Filter** — a condition tree built from `Equal`, `Not`, `And`, and `Or` nodes that decides which rows to keep
- **Group by** — a column to group the filtered rows by
- **Aggregation** — `Count`, `Sum`, or `Average` over a column within each group

The server evaluates the query against the dataset and returns the result as a new dataset. The query struct and all its components are serializable via `serde`, so they pass cleanly over the wire.

There are two RPCs:

- `slow_rpc` — returns the entire dataset as-is (no query)
- `fast_rpc` — takes a `Query`, runs `compute_query_on_dataset`, returns the result

---

## Running

Start the server with either dataset:

```bash
cargo run -- grades
cargo run -- albums
```

The server looks for `../grades.csv` or `../albums.csv` relative to the binary, so make sure you're running from the right directory.

---

## Datasets

**grades.csv** — student grade records with columns for student, assignment, and score.

**albums.csv** — 10,000 randomly generated album ratings for three metal bands: Meshuggah, Vildhjarta, and Humanity's Last Breath. Each album has a mean rating baked in (e.g. *Catch Thirtythree* and *ObZen* both have mean 5), and individual ratings are sampled from a normal distribution with σ=2, clamped at 0.

To regenerate the albums dataset:

```bash
python3 albums.py > albums.csv
```

---

## Query structure

Queries are defined in `query.rs`. A `Query` wraps a `Condition`, a group-by column name, and an `Aggregation`:

```
Query {
    filter: Condition,   // Equal / Not / And / Or
    group_by: String,    // column to group by
    aggregate: Aggregation  // Count / Sum / Average over a column
}
```

The result column in the output dataset is automatically named based on the aggregation, e.g. `Count(rating)` or `Average(score)`.

---

## File overview

```
├── main.rs         # entry point — parses args, loads CSV, starts server
├── solution.rs     # RPC handlers: hello, slow_rpc, fast_rpc
├── query.rs        # Query, Condition, and Aggregation types
├── dataset.rs      # Dataset and Value types
├── csv.rs          # CSV parsing into a Dataset
├── filter.rs       # evaluates a Condition against a row
├── group_by.rs     # groups rows by a column value
├── aggregate.rs    # computes Count/Sum/Average over grouped rows
├── lib.rs          # library exports
├── albums.py       # generates albums.csv (10k rows, normal dist ratings)
├── albums.csv      # pre-generated album ratings dataset
├── grades.csv      # student grades dataset
└── Cargo.toml
```

The core query logic lives across `filter.rs`, `group_by.rs`, and `aggregate.rs` — they run in sequence: filter rows → group → aggregate.

---

## Notes

- The dataset is passed to the server as a `&'static Dataset` via `Box::leak`, which gives it a static lifetime without wrapping it in a mutex. Since the dataset is read-only after load, this is safe.
- Queries are fully composable — conditions can be arbitrarily nested using `And`, `Or`, and `Not`.
- The Python script uses `random.normalvariate` for ratings, so the generated CSV will differ on each run. The means per album are hardcoded in `albums.py`.
