# Fretboard Layout
<!-- cargo-sync-readme start -->

`fretboard_layout` is a library for turning a set of specifications into a
complete template of a stringed musical instrument fretboard, such as a
guitar, banjo, or mandolin.
| ![Sample output](https://jeang3nie.codeberg.page/gfret/sample.svg) |
| :--: |
| **Sample output** |
## Usage
```rust
use fretboard_layout::{Config,Specs};

    // the [Specs] struct constains the specifications used to generate the svg
    let mut specs = Specs::default();
    specs.set_multi(Some(615.0));
    specs.set_scale(675.0);
    // the (optional) [Config] struct fine tunes the visual representation
    let mut cfg = Config::default();
    cfg.set_line_weight(0.5);
    let svg = specs.create_document(Some(cfg));
```

<!-- cargo-sync-readme end -->
