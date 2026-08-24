use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        single: { not(any(feature = "compute", feature = "threads")) },
    }
}
