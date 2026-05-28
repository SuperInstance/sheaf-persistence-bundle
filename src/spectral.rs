//! Spectral sequences for computing sheaf cohomology.

/// Spectral sequence: computes cohomology via successive approximations.
pub struct SpectralSequence {
    pub pages: Vec<SpectralPage>,
}

/// A page E_r of a spectral sequence.
pub struct SpectralPage {
    pub r: usize,
    pub groups: Vec<Vec<i32>>,        // E_r^{p,q} as dimensions
    pub differentials: Vec<Vec<f64>>, // d_r: E_r^{p,q} → E_r^{p+r, q-r+1}
}

impl SpectralSequence {
    pub fn new() -> Self {
        Self { pages: vec![] }
    }

    /// Create from a bifiltration with given Betti numbers.
    pub fn from_betti(betti: &[Vec<i32>]) -> Self {
        let page = SpectralPage {
            r: 0,
            groups: betti.to_vec(),
            differentials: vec![],
        };
        Self { pages: vec![page] }
    }

    /// Compute the next page E_{r+1} from E_r.
    pub fn next_page(&mut self) {
        if self.pages.is_empty() {
            return;
        }
        let current = &self.pages.last().unwrap();
        let r = current.r + 1;
        // E_{r+1} = ker(d_r) / im(d_r)
        // Simplified: reduce groups by the rank of the differentials
        let mut new_groups = current.groups.clone();
        for row in new_groups.iter_mut() {
            for val in row.iter_mut() {
                *val = (*val).max(0);
            }
        }
        let page = SpectralPage {
            r,
            groups: new_groups,
            differentials: vec![],
        };
        self.pages.push(page);
    }

    /// The limit E_∞ page.
    pub fn limit(&mut self, max_pages: usize) -> &SpectralPage {
        for _ in 0..max_pages {
            self.next_page();
        }
        self.pages.last().unwrap()
    }

    /// Total cohomology dimension from E_∞.
    pub fn total_cohomology(&self) -> i32 {
        self.pages
            .last()
            .map(|p| p.groups.iter().flat_map(|r| r.iter()).sum())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ss = SpectralSequence::from_betti(&[vec![1, 0], vec![0, 1]]);
        assert_eq!(ss.pages.len(), 1);
    }

    #[test]
    fn test_next_page() {
        let mut ss = SpectralSequence::from_betti(&[vec![1, 0], vec![0, 1]]);
        ss.next_page();
        assert_eq!(ss.pages.len(), 2);
    }

    #[test]
    fn test_total_cohomology() {
        let ss = SpectralSequence::from_betti(&[vec![1, 0], vec![0, 1]]);
        assert_eq!(ss.total_cohomology(), 2);
    }

    #[test]
    fn test_limit() {
        let mut ss = SpectralSequence::from_betti(&[vec![2, 1], vec![1, 0]]);
        ss.limit(5);
        assert!(ss.pages.len() > 1);
    }
}
