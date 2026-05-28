//! Multi-parameter persistence.

/// Multi-parameter persistence: tracks topological features across multiple filtration parameters.
pub struct MultiparameterPersistence {
    pub dimensions: usize,
    pub grades: Vec<Vec<f64>>,
    pub modules: Vec<PersistenceModule>,
}

/// A persistence module over a multi-parameter grading.
pub struct PersistenceModule {
    pub birth: Vec<f64>,
    pub death: Vec<f64>,
    pub dimension: usize,
}

impl MultiparameterPersistence {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            grades: vec![],
            modules: vec![],
        }
    }

    /// Add a persistence module (birth-death pair in multi-parameter space).
    pub fn add_module(&mut self, birth: Vec<f64>, death: Vec<f64>, dimension: usize) {
        self.modules.push(PersistenceModule {
            birth,
            death,
            dimension,
        });
    }

    /// Number of alive modules at a given grade.
    pub fn betti_number(&self, grade: &[f64], dimension: usize) -> usize {
        self.modules
            .iter()
            .filter(|m| {
                m.dimension == dimension
                    && m.birth.iter().zip(grade).all(|(b, g)| b <= g)
                    && m.death.iter().zip(grade).all(|(d, g)| d > g)
            })
            .count()
    }

    /// Rank invariant: the number of features alive between two grades.
    pub fn rank_invariant(&self, grade1: &[f64], grade2: &[f64], dimension: usize) -> usize {
        self.modules
            .iter()
            .filter(|m| {
                m.dimension == dimension
                    && m.birth.iter().zip(grade1).all(|(b, g)| b <= g)
                    && m.death.iter().zip(grade2).all(|(d, g)| d > g)
            })
            .count()
    }

    /// Total persistence across all parameters.
    pub fn total_persistence(&self, power: f64) -> f64 {
        self.modules
            .iter()
            .map(|m| {
                let persist: f64 = m
                    .birth
                    .iter()
                    .zip(&m.death)
                    .map(|(b, d)| (d - b).abs())
                    .sum();
                persist.powf(power)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mp = MultiparameterPersistence::new(2);
        assert_eq!(mp.betti_number(&[0.5, 0.5], 0), 0);
    }

    #[test]
    fn test_add_module() {
        let mut mp = MultiparameterPersistence::new(2);
        mp.add_module(vec![0.0, 0.0], vec![1.0, 1.0], 0);
        assert_eq!(mp.modules.len(), 1);
    }

    #[test]
    fn test_betti_number() {
        let mut mp = MultiparameterPersistence::new(2);
        mp.add_module(vec![0.0, 0.0], vec![2.0, 2.0], 0);
        assert_eq!(mp.betti_number(&[1.0, 1.0], 0), 1);
        assert_eq!(mp.betti_number(&[3.0, 3.0], 0), 0);
    }

    #[test]
    fn test_total_persistence() {
        let mut mp = MultiparameterPersistence::new(2);
        mp.add_module(vec![0.0, 0.0], vec![1.0, 1.0], 0);
        let tp = mp.total_persistence(1.0);
        assert!((tp - 2.0).abs() < 1e-10);
    }
}
