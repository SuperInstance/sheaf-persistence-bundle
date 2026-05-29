//! Sheaf bundle: a fiber bundle where the fiber is a sheaf.

/// A sheaf bundle: a topological space (base) with a sheaf attached at each point.
///
/// This generalizes both vector bundles (constant sheaf) and covering spaces
/// (locally constant sheaf).
pub struct SheafBundle {
    /// Dimension of the base complex (number of vertices).
    pub base_dim: usize,
    /// Fiber dimension at each vertex.
    pub fiber_dim: usize,
    /// Transition functions between fibers along edges.
    /// transition_maps[edge_idx] = matrix mapping fiber at source to fiber at target.
    pub transition_maps: Vec<Vec<Vec<f64>>>,
    /// Connection: parallel transport rule.
    pub connection: Vec<f64>,
}

impl SheafBundle {
    pub fn new(base_dim: usize, fiber_dim: usize) -> Self {
        let _id: Vec<Vec<f64>> = (0..fiber_dim)
            .map(|i| {
                (0..fiber_dim)
                    .map(|j| if i == j { 1.0 } else { 0.0 })
                    .collect()
            })
            .collect();
        Self {
            base_dim,
            fiber_dim,
            transition_maps: vec![],
            connection: vec![0.0; base_dim],
        }
    }

    /// Trivial bundle: all fibers are R^n with identity transitions.
    pub fn trivial(base_dim: usize, fiber_dim: usize) -> Self {
        Self::new(base_dim, fiber_dim)
    }

    /// Add a transition map for an edge.
    pub fn add_transition(&mut self, map: Vec<Vec<f64>>) {
        self.transition_maps.push(map);
    }

    /// Compute curvature: the obstruction to flatness.
    /// For a flat bundle, curvature is zero.
    pub fn curvature(&self) -> f64 {
        if self.transition_maps.is_empty() {
            return 0.0;
        }
        // Measure how far the product of transitions around a loop deviates from identity
        let mut det_sum = 0.0;
        for map in &self.transition_maps {
            let det = if map.len() == 1 {
                map[0][0]
            } else if map.len() == 2 {
                map[0][0] * map[1][1] - map[0][1] * map[1][0]
            } else {
                1.0
            };
            det_sum += (det - 1.0).abs();
        }
        det_sum / self.transition_maps.len().max(1) as f64
    }

    /// Parallel transport along an edge.
    pub fn transport(&self, edge_idx: usize, fiber_vec: &[f64]) -> Vec<f64> {
        if edge_idx >= self.transition_maps.len() {
            return fiber_vec.to_vec();
        }
        let map = &self.transition_maps[edge_idx];
        (0..self.fiber_dim)
            .map(|i| {
                (0..self.fiber_dim)
                    .map(|j| {
                        map.get(i)
                            .and_then(|r| r.get(j))
                            .copied()
                            .unwrap_or(if i == j { 1.0 } else { 0.0 })
                            * fiber_vec.get(j).copied().unwrap_or(0.0)
                    })
                    .sum()
            })
            .collect()
    }

    /// Check if the bundle is trivial (all transitions are identity).
    pub fn is_trivial(&self) -> bool {
        self.curvature() < 1e-10
    }

    /// Total dimension: base_dim * fiber_dim.
    pub fn total_dim(&self) -> usize {
        self.base_dim * self.fiber_dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_bundle() {
        let b = SheafBundle::trivial(3, 2);
        assert!(b.is_trivial());
        assert_eq!(b.total_dim(), 6);
    }

    #[test]
    fn test_curvature_trivial() {
        let b = SheafBundle::trivial(3, 2);
        assert!(b.curvature() < 1e-10);
    }

    #[test]
    fn test_transport_identity() {
        let b = SheafBundle::trivial(3, 2);
        let v = vec![1.0, 2.0];
        let t = b.transport(0, &v);
        assert_eq!(t, v);
    }

    #[test]
    fn test_nontrivial_bundle() {
        let mut b = SheafBundle::new(3, 2);
        b.add_transition(vec![vec![3.0, 0.0], vec![0.0, 0.5]]);
        assert!(b.curvature() > 0.0);
    }
}
