# ternary-percolation

Percolation theory primitives for ternary {-1, 0, +1} lattices. Flood-fill cluster counting, spanning detection, critical density estimation via binary search, and percolation strength — all in pure Rust with zero dependencies.

## Why It Matters

Percolation describes the emergence of long-range connectivity in disordered media. For ternary systems, this means understanding when and how a population of {-1, 0, +1} agents forms a connected path across a spatial domain. The critical density — the tipping point where connectivity suddenly appears — is a phase transition with universal scaling properties.

This crate provides:
- **Flood-fill clustering**: identify all connected regions of a given ternary state
- **Spanning detection**: does any connected path cross the entire lattice?
- **Critical density estimation**: binary search for the percolation threshold $p_c$
- **Percolation strength**: fraction of the system in the largest cluster

Applications include agent network formation, ternary sensor field analysis, opinion dynamics on spatial grids, and studying phase transitions in three-state Potts-like models.

## How It Works

### Grid Model

Grids are stored row-major: $\text{index} = y \times W + x$. Each cell holds a value $v \in \{-1, 0, +1\}$.

### Random Filling

Fill with density $p$:

$$v_i = \begin{cases} +1 & \text{if } r_i < p \text{ and } u_i < 0.5 \\ -1 & \text{if } r_i < p \text{ and } u_i \geq 0.5 \\ 0 & \text{otherwise} \end{cases}$$

where $r_i, u_i$ are uniform random variates. The expected number of non-zero cells is $p \times W \times H$.

### Spanning Detection (DFS Flood Fill)

Starting from each cell with value $v$ on the top row, perform depth-first search through 4-connected neighbors of matching value. If the search reaches the bottom row, the grid percolates for state $v$.

**Complexity:** O($N$) where $N = W \times H$ — each cell visited once.

### Cluster Counting

Enumerate all connected components using iterative DFS:

1. Scan for unvisited cell with target value
2. Flood fill to mark entire component
3. Record component and increment count
4. Repeat

**Complexity:** O($N$).

### Largest Cluster

Same as cluster counting, but track the maximum component size encountered.

### Critical Density

Binary search over $p \in [0, 1]$:

1. For midpoint $p_{\text{mid}}$, generate `samples` random grids
2. Count fraction that percolate
3. If fraction < 0.5: search upper half ($p_c > p_{\text{mid}}$)
4. If fraction ≥ 0.5: search lower half

**Complexity:** O($N \cdot \text{samples} \cdot \log_2(1/\epsilon)$) for precision $\epsilon$.

**Known result:** for 2D square lattice site percolation, $p_c \approx 0.5928$.

### Percolation Strength

$$P_\infty = \frac{S_{\max}}{N}$$

where $S_{\max}$ is the largest cluster size and $N$ is the total number of cells.

Near $p_c$, $P_\infty \sim (p - p_c)^\beta$ with $\beta = 5/36$ in 2D (universality class of the Potts model with $q = 1$, i.e., percolation).

## Quick Start

```rust
use ternary_percolation::*;

// Deterministic RNG for reproducibility
let mut s: u64 = 42;
let mut rng = || {
    s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
    (s >> 33) as f64 / (1u64 << 31) as f64
};

// Fill a random grid
let grid = fill_grid(20, 20, 0.5, &mut rng);
assert_eq!(grid.len(), 400);

// Check percolation for +1
let percs = percolates(&grid, 20, 1);

// Count clusters
let nc = cluster_count(&grid, 20, 1);

// Largest cluster
let largest = largest_cluster(&grid, 20, 1);

// Percolation strength
let strength = percolation_strength(&grid, 20, 1);

// Critical density estimation
let pc = critical_density(10, 10, 30, &mut rng);
println!("Critical density ≈ {:.3}", pc);
```

## API

| Function | Description |
|---|---|
| `fill_grid(w, h, density, rng) → Vec<i8>` | Random ternary grid |
| `percolates(grid, w, target) → bool` | DFS spanning check top→bottom |
| `cluster_count(grid, w, target) → usize` | Number of connected components |
| `largest_cluster(grid, w, target) → usize` | Size of biggest component |
| `percolation_strength(grid, w, target) → f64` | $S_{\max} / N$ |
| `critical_density(w, h, samples, rng) → f64` | Binary search for $p_c$ |

## Architecture Notes

The ternary percolation model maps directly onto the **γ + η = C** conservation identity. The +1 sites represent constructive mass γ, the -1 sites represent inhibitory mass η, and the 0 sites are neutral. The percolation of +1 or -1 clusters corresponds to the formation of long-range order in the γ or η population — a spatial phase transition.

Because the total site count is conserved ($C = W \times H$), increasing γ percolation necessarily means decreasing the available sites for η percolation. The competition between γ-clusters and η-clusters near the percolation threshold creates rich critical phenomena analogous to the $q = 3$ Potts model, where the universal exponents differ from simple ($q = 1$) percolation: $\beta = 1/9$ for the 3-state Potts universality class.

## References

- Stauffer, D. & Aharony, A. (2018). *Introduction to Percolation Theory.* 2nd ed. CRC Press.
- Wu, F. Y. (1982). *The Potts Model.* Reviews of Modern Physics, 54(1). (q-state universality)
- Isichenko, M. B. (1992). *Percolation, statistical topography, and transport in random media.* Rev. Mod. Phys., 64(4).
- Newman, M. E. J. & Ziff, R. M. (2000). *Efficient Monte Carlo Algorithm and High-Precision Results for Percolation.* Phys. Rev. Lett., 85.

## License

MIT
