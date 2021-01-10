Attempting to complete Advent of Code 2020 in Rust, written, compiled & run entirely on my Raspberry Pi.

Mix of good & bad code depending on the day.

Most days run in well under 1s.  Some days are taking longer - could be interesting to investigate/ improve these.

| Day | Debug | Release | Notes
| --- | --- | --- | ---
| 11-1 | ~20s  | ~1s  |
| 11-2 | ~20s  | ~1s  |
| 15-2 | ~180s | ~15s | Test shows this is linear on input size - so optimization would need to cut down time per iteration, or see if there is a shortcut algorithm that can go straight to the answer.
| 17-1 |       | 0.5s |
| 17-2 |       | ~18s | Improvement: memoize the `get_neighbours` function.
| 22-2 | ~60s  | 2.9s |
| 23-2 | ~12s  | 1.6s | 
