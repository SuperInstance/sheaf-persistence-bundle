//! Cross-modal data fusion via sheaf persistence.

/// Cross-modal fusion: combines multiple data modalities into a unified topological representation.
pub struct CrossModalFusion {
    pub n_modalities: usize,
    pub stalk_dims: Vec<usize>,
}

impl CrossModalFusion {
    pub fn new(stalk_dims: Vec<usize>) -> Self {
        let n = stalk_dims.len();
        Self {
            n_modalities: n,
            stalk_dims,
        }
    }

    /// Total stalk dimension (sum of all modality dimensions).
    pub fn total_stalk_dim(&self) -> usize {
        self.stalk_dims.iter().sum()
    }

    /// Fusion weight for each modality.
    pub fn fusion_weights(&self, modality_reliability: &[f64]) -> Vec<f64> {
        let total: f64 = modality_reliability.iter().sum();
        modality_reliability.iter().map(|r| r / total).collect()
    }

    /// Weighted distance combining multiple modalities.
    pub fn fused_distance(&self, distances: &[Vec<Vec<f64>>], weights: &[f64]) -> Vec<Vec<f64>> {
        let n = distances.first().map(|d| d.len()).unwrap_or(0);
        let mut fused = vec![vec![0.0; n]; n];
        for (d, &w) in distances.iter().zip(weights) {
            for i in 0..n {
                for j in 0..n {
                    fused[i][j] += w * d[i][j] * d[i][j];
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                fused[i][j] = fused[i][j].sqrt();
            }
        }
        fused
    }

    /// Consistency check: are the modalities compatible?
    pub fn consistency(&self, distances: &[Vec<Vec<f64>>]) -> f64 {
        if distances.len() < 2 {
            return 1.0;
        }
        let n = distances[0].len();
        let mut total_diff = 0.0;
        let mut count = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let vals: Vec<f64> = distances.iter().map(|d| d[i][j]).collect();
                let mean: f64 = vals.iter().sum::<f64>() / vals.len() as f64;
                let var: f64 =
                    vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
                total_diff += var.sqrt();
                count += 1;
            }
        }
        if count == 0 {
            return 1.0;
        }
        1.0 / (1.0 + total_diff / count as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let f = CrossModalFusion::new(vec![3, 2, 4]);
        assert_eq!(f.n_modalities, 3);
        assert_eq!(f.total_stalk_dim(), 9);
    }

    #[test]
    fn test_fusion_weights() {
        let f = CrossModalFusion::new(vec![2, 2]);
        let w = f.fusion_weights(&[0.8, 0.2]);
        assert!((w[0] - 0.8).abs() < 1e-10);
        assert!((w[1] - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_fused_distance() {
        let f = CrossModalFusion::new(vec![2, 2]);
        let d1 = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let d2 = vec![vec![0.0, 3.0], vec![3.0, 0.0]];
        let fused = f.fused_distance(&[d1, d2], &[0.5, 0.5]);
        assert!(fused[0][1] > 0.0);
    }

    #[test]
    fn test_consistency_identical() {
        let f = CrossModalFusion::new(vec![2, 2]);
        let d = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let c = f.consistency(&[d.clone(), d]);
        assert!(c > 0.9);
    }
}
