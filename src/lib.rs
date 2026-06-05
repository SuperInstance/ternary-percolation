#![forbid(unsafe_code)]

/// Percolation on ternary grids. Values are -1, 0, or +1.
/// Grid is stored row-major: index = row * width + col.

/// Fill a grid with random ternary values.
/// density controls probability of non-zero; non-zero values are ±1 with equal chance.
pub fn fill_grid(width: usize, height: usize, density: f64, rng: &mut impl FnMut() -> f64) -> Vec<i8> {
    let mut grid = vec![0i8; width * height];
    for cell in grid.iter_mut() {
        if rng() < density {
            *cell = if rng() < 0.5 { -1 } else { 1 };
        }
    }
    grid
}

/// Check if the grid percolates top-to-bottom for the given target value.
/// Uses flood fill from the top row.
pub fn percolates(grid: &[i8], width: usize, target_value: i8) -> bool {
    if grid.is_empty() || width == 0 {
        return false;
    }
    let height = grid.len() / width;
    if height == 0 {
        return false;
    }

    let mut visited = vec![false; grid.len()];
    let mut stack = Vec::new();

    // Seed from top row
    for col in 0..width {
        if grid[col] == target_value {
            visited[col] = true;
            stack.push(col);
        }
    }

    while let Some(idx) = stack.pop() {
        let row = idx / width;
        let col = idx % width;

        // Reached bottom
        if row == height - 1 {
            return true;
        }

        // Check 4 neighbors
        let neighbors = [
            if row > 0 { Some(idx - width) } else { None },
            if row + 1 < height { Some(idx + width) } else { None },
            if col > 0 { Some(idx - 1) } else { None },
            if col + 1 < width { Some(idx + 1) } else { None },
        ];

        for n in neighbors.iter().flatten() {
            if !visited[*n] && grid[*n] == target_value {
                visited[*n] = true;
                stack.push(*n);
            }
        }
    }

    false
}

/// Count the number of distinct clusters of target_value using flood fill.
pub fn cluster_count(grid: &[i8], width: usize, target_value: i8) -> usize {
    if grid.is_empty() || width == 0 {
        return 0;
    }
    let height = grid.len() / width;
    let mut visited = vec![false; grid.len()];
    let mut count = 0;

    for start in 0..grid.len() {
        if visited[start] || grid[start] != target_value {
            continue;
        }
        count += 1;
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(idx) = stack.pop() {
            let row = idx / width;
            let col = idx % width;
            let neighbors = [
                if row > 0 { Some(idx - width) } else { None },
                if row + 1 < height { Some(idx + width) } else { None },
                if col > 0 { Some(idx - 1) } else { None },
                if col + 1 < width { Some(idx + 1) } else { None },
            ];
            for n in neighbors.iter().flatten() {
                if !visited[*n] && grid[*n] == target_value {
                    visited[*n] = true;
                    stack.push(*n);
                }
            }
        }
    }

    count
}

/// Size of the largest cluster of target_value.
pub fn largest_cluster(grid: &[i8], width: usize, target_value: i8) -> usize {
    if grid.is_empty() || width == 0 {
        return 0;
    }
    let height = grid.len() / width;
    let mut visited = vec![false; grid.len()];
    let mut max_size = 0;

    for start in 0..grid.len() {
        if visited[start] || grid[start] != target_value {
            continue;
        }
        let mut size = 0;
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(idx) = stack.pop() {
            size += 1;
            let row = idx / width;
            let col = idx % width;
            let neighbors = [
                if row > 0 { Some(idx - width) } else { None },
                if row + 1 < height { Some(idx + width) } else { None },
                if col > 0 { Some(idx - 1) } else { None },
                if col + 1 < width { Some(idx + 1) } else { None },
            ];
            for n in neighbors.iter().flatten() {
                if !visited[*n] && grid[*n] == target_value {
                    visited[*n] = true;
                    stack.push(*n);
                }
            }
        }
        if size > max_size {
            max_size = size;
        }
    }

    max_size
}

/// Estimate the critical density via binary search over percolation probability.
pub fn critical_density(width: usize, height: usize, samples: usize, rng: &mut impl FnMut() -> f64) -> f64 {
    let mut lo = 0.0;
    let mut hi = 1.0;

    for _ in 0..15 {
        let mid = (lo + hi) / 2.0;
        let mut percolation_count = 0;
        for _ in 0..samples {
            let grid = fill_grid(width, height, mid, rng);
            if percolates(&grid, width, 1) {
                percolation_count += 1;
            }
        }
        let prob = percolation_count as f64 / samples as f64;
        if prob < 0.5 {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    (lo + hi) / 2.0
}

/// Fraction of cells belonging to the percolating cluster (or largest if no percolation).
pub fn percolation_strength(grid: &[i8], width: usize, target_value: i8) -> f64 {
    if grid.is_empty() {
        return 0.0;
    }
    let largest = largest_cluster(grid, width, target_value);
    largest as f64 / grid.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deterministic_rng(seed: u64) -> impl FnMut() -> f64 {
        let mut s = seed;
        move || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            (s >> 33) as f64 / (1u64 << 31) as f64
        }
    }

    #[test]
    fn test_fill_grid_size() {
        let mut rng = deterministic_rng(42);
        let grid = fill_grid(5, 3, 0.5, &mut rng);
        assert_eq!(grid.len(), 15);
    }

    #[test]
    fn test_fill_grid_values() {
        let mut rng = deterministic_rng(42);
        let grid = fill_grid(5, 3, 1.0, &mut rng);
        for &v in &grid {
            assert!(v == -1 || v == 0 || v == 1);
        }
    }

    #[test]
    fn test_fill_grid_zero_density() {
        let mut rng = deterministic_rng(42);
        let grid = fill_grid(4, 4, 0.0, &mut rng);
        assert!(grid.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_percolates_simple_yes() {
        // 3x3 grid, all 1s — should percolate
        let grid = vec![1i8; 9];
        assert!(percolates(&grid, 3, 1));
    }

    #[test]
    fn test_percolates_simple_no() {
        // 3x3 grid with middle row blocked
        let grid = vec![1, 1, 1, 0, 0, 0, 1, 1, 1i8];
        assert!(!percolates(&grid, 3, 1));
    }

    #[test]
    fn test_percolates_empty() {
        assert!(!percolates(&[], 0, 1));
    }

    #[test]
    fn test_cluster_count_single() {
        let grid = vec![1i8; 4]; // 2x2 all 1s
        assert_eq!(cluster_count(&grid, 2, 1), 1);
    }

    #[test]
    fn test_cluster_count_none() {
        let grid = vec![0i8; 4];
        assert_eq!(cluster_count(&grid, 2, 1), 0);
    }

    #[test]
    fn test_cluster_count_multiple() {
        // 1 0 1
        // 0 0 0
        // 1 0 1
        let grid = vec![1, 0, 1, 0, 0, 0, 1, 0, 1i8];
        assert_eq!(cluster_count(&grid, 3, 1), 4);
    }

    #[test]
    fn test_largest_cluster_all() {
        let grid = vec![1i8; 9];
        assert_eq!(largest_cluster(&grid, 3, 1), 9);
    }

    #[test]
    fn test_largest_cluster_empty() {
        assert_eq!(largest_cluster(&[], 0, 1), 0);
    }

    #[test]
    fn test_percolation_strength_all() {
        let grid = vec![1i8; 9];
        let s = percolation_strength(&grid, 3, 1);
        assert!((s - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_percolation_strength_partial() {
        // 2x2 grid, only top-left is 1
        let grid = vec![1, 0, 0, 0i8];
        let s = percolation_strength(&grid, 2, 1);
        assert!((s - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_critical_density_range() {
        let mut rng = deterministic_rng(42);
        let cd = critical_density(5, 5, 20, &mut rng);
        assert!(cd >= 0.0 && cd <= 1.0);
    }

    #[test]
    fn test_fill_grid_high_density() {
        let mut rng = deterministic_rng(42);
        let grid = fill_grid(10, 10, 0.99, &mut rng);
        let non_zero = grid.iter().filter(|&&v| v != 0).count();
        assert!(non_zero > 80); // Almost all should be non-zero
    }

    #[test]
    fn test_percolates_negative() {
        // Test with target -1
        let grid = vec![-1i8; 9];
        assert!(percolates(&grid, 3, -1));
    }
}
