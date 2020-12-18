Attempting to complete Advent of Code 2020 in Rust, written, compiled & run entirely on my Raspberry Pi.

Mix of good & bad code depending on the day.

Most days run in well under 1s.  Some days are taking longer - could be interesting to investigate/ improve these.

- day 11 : parts one and two take ~20s each (debug) / ~1s each (release).
- day 15 : part two takes ~3 min on my Pi (though only 15s with `--release`).  Test shows this is linear on input size - so optimization would need to cut down time per iteration, or see if there is a shortcut algorithm that can go straight to the answer.
- day 17 : 0.5s for part one, 18s for part two (both in release mode).  Improvement: memoize the `get_neighbours` function.
