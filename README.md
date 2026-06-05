# ternary-percolation

**Percolation theory on ternary grids. When does the path connect?**

Percolation asks a simple question: if you randomly fill a grid with active cells, at what density does a connected path form from one side to the other? Below the critical threshold, you get isolated islands. Above it, a spanning cluster emerges. The transition is sharp — a genuine phase transition.

For ternary grids, the question gets richer: you can percolate *either* the +1 cells *or* the -1 cells (or treat 0 as the barrier). This crate implements flood-fill percolation detection, critical density estimation, and cluster analysis on ternary grids.

## What's Inside

- **`fill_grid(width, height, density, rng)`** — generate a random ternary grid with given density
- **`percolates(grid, width, target_value)`** — does the target value connect top to bottom? Flood-fill from top row
- **`find_critical_density(width, height, target_value, rng, steps)`** — binary search for the percolation threshold
- **`largest_cluster(grid, width, target_value)`** — size of the largest connected component
- **`cluster_sizes(grid, width, target_value)`** — all cluster sizes (sorted descending)
- **`percolation_probability(width, height, density, target_value, trials, rng)`** — fraction of random grids that percolate at given density

## Quick Example

```rust
use ternary_percolation::*;

let mut rng = || 0.42; // your RNG

// Random grid with 40% density
let grid = fill_grid(20, 20, 0.4, &mut rng);

// Does +1 percolate top to bottom?
if percolates(&grid, 20, 1) {
    println!("+1 cells form a spanning cluster!");
}

// Find the critical density
let critical = find_critical_density(20, 20, 1, &mut rng, 20);
println!("Critical density: {:.3}", critical);
// Expected: ~0.59 for site percolation on square lattice

// Cluster analysis
let sizes = cluster_sizes(&grid, 20, 1);
println!("Largest cluster: {} cells", sizes[0]);
```

## The Deeper Truth

**Percolation is the bridge between local and global.** No individual cell "knows" about the spanning cluster. Each cell is independently active or not. But above the critical density, a macroscopic connected structure *emerges* from purely local decisions. This is the same mechanism behind epidemic spreading, network connectivity, and material porosity.

For ternary systems, percolation has a unique wrinkle: you can ask whether +1 percolates, whether -1 percolates, or whether they *both* percolate simultaneously. At moderate densities, they can't both percolate — there isn't enough space. This creates a competition between +1 and -1 clusters, with 0 cells as the contested territory.

The critical density for ternary site percolation on a square lattice is ~12% — lower than binary percolation (~59%) because ternary grids have three competing phases, so each phase needs fewer cells to span.

**Use cases:**
- **Network resilience** — at what failure rate does the network disconnect?
- **Epidemic modeling** — what infection rate causes pandemic spread?
- **Material science** — porosity and conductivity thresholds
- **Image segmentation** — connected component extraction
- **Infrastructure planning** — connectivity requirements for distributed systems

## See Also

- **ternary-fire** — fire spreading is percolation with dynamics
- **ternary-minority** — the anti-percolation force (prevents spanning clusters)
- **ternary-morph** — morphological reconstruction (related to connected components)
- **ternary-field** — gradient analysis of percolation clusters
- **ternary-drift** — how percolation changes under drift dynamics

## Install

```bash
cargo add ternary-percolation
```

## License

MIT
