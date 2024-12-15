use std::collections::HashMap;
use petgraph::graph::{NodeIndex};

/// Function that normalizes centrality values between 0 and 1
pub fn normalize_centrality_values(centrality: &HashMap<NodeIndex, f64>) -> HashMap<NodeIndex, f64> {
    let max_centrality = centrality.values().cloned().fold(0.0, f64::max);
    centrality
        .iter()
        .map(|(node, &value)| (*node, value / max_centrality))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normalization() {
        let mut centrality = HashMap::new();
        centrality.insert(NodeIndex::new(0), 100.0);
        centrality.insert(NodeIndex::new(1), 200.0);
        centrality.insert(NodeIndex::new(2), 50.0);

        let normalized = normalize_centrality_values(&centrality);
        
        // The max centrality is 200, so normalized values should be between 0 and 1
        assert_eq!(normalized[&NodeIndex::new(0)], 0.5);  // 100 / 200
        assert_eq!(normalized[&NodeIndex::new(1)], 1.0);  // 200 / 200
        assert_eq!(normalized[&NodeIndex::new(2)], 0.25); // 50 / 200
    }
}