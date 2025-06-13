use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FuzzySet<T> {
    pub category: T,
    pub points: Vec<(f32, f32)>, // (input_value, membership_degree)
}

impl<T> FuzzySet<T> {
    pub fn new(category: T, points: Vec<(f32, f32)>) -> Self {
        Self { category, points }
    }

    pub fn membership(&self, input: f32) -> f32 {
        // Handle edge cases
        if self.points.is_empty() {
            return 0.0;
        }

        if input <= self.points[0].0 {
            return self.points[0].1;
        }
        if input >= self.points.last().unwrap().0 {
            return self.points.last().unwrap().1;
        }

        // Find the two points to interpolate between
        for i in 0..self.points.len() - 1 {
            let (x1, y1) = self.points[i];
            let (x2, y2) = self.points[i + 1];

            if input >= x1 && input <= x2 {
                if x2 == x1 {
                    return y1;
                }
                // Linear interpolation
                return y1 + (y2 - y1) * (input - x1) / (x2 - x1);
            }
        }

        0.0
    }
}

#[derive(Debug, Clone)]
pub struct FuzzyRule<InputType, OutputType> {
    pub condition: InputType,
    pub consequences: HashMap<OutputType, f32>,
}

impl<InputType, OutputType> FuzzyRule<InputType, OutputType>
where
    InputType: Clone,
    OutputType: Clone + Eq + std::hash::Hash,
{
    pub fn new(condition: InputType) -> Self {
        Self {
            condition,
            consequences: HashMap::new(),
        }
    }

    pub fn with_consequence(mut self, output: OutputType, value: f32) -> Self {
        self.consequences.insert(output, value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct FuzzySystem<InputType, OutputType> {
    pub input_sets: Vec<FuzzySet<InputType>>,
    pub rules: Vec<FuzzyRule<InputType, OutputType>>,
}

impl<InputType, OutputType> Default for FuzzySystem<InputType, OutputType> {
    fn default() -> Self {
        Self {
            input_sets: Vec::new(),
            rules: Vec::new(),
        }
    }
}

impl<InputType, OutputType> FuzzySystem<InputType, OutputType>
where
    InputType: Clone + PartialEq,
    OutputType: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            input_sets: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_input_set(&mut self, set: FuzzySet<InputType>) {
        self.input_sets.push(set);
    }

    pub fn add_rule(&mut self, rule: FuzzyRule<InputType, OutputType>) {
        self.rules.push(rule);
    }

    pub fn evaluate(&self, input_value: f32) -> HashMap<OutputType, f32> {
        let mut weighted_outputs: HashMap<OutputType, Vec<(f32, f32)>> = HashMap::new();

        // For each rule, calculate its strength and apply consequences
        for rule in &self.rules {
            // Find membership degree for this rule's condition
            let rule_strength = self
                .input_sets
                .iter()
                .find(|set| set.category == rule.condition)
                .map(|set| set.membership(input_value))
                .unwrap_or(0.0);

            if rule_strength > 0.0 {
                for (output_param, value) in &rule.consequences {
                    weighted_outputs
                        .entry(output_param.clone())
                        .or_default()
                        .push((*value, rule_strength));
                }
            }
        }

        // Calculate weighted averages
        let mut result = HashMap::new();
        for (param, weighted_values) in weighted_outputs {
            let total_weight: f32 = weighted_values.iter().map(|(_, w)| w).sum();
            let weighted_sum: f32 = weighted_values.iter().map(|(v, w)| v * w).sum();

            if total_weight > 0.0 {
                result.insert(param, weighted_sum / total_weight);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test FuzzySet functionality
    #[test]
    fn test_fuzzy_set_creation() {
        let points = vec![(0.0, 0.0), (5.0, 1.0), (10.0, 0.0)];
        let set = FuzzySet::new("temperature", points.clone());

        assert_eq!(set.category, "temperature");
        assert_eq!(set.points, points);
    }

    #[test]
    fn test_membership_empty_points() {
        let set = FuzzySet::new("empty", vec![]);
        assert_eq!(set.membership(5.0), 0.0);
    }

    #[test]
    fn test_membership_single_point() {
        let set = FuzzySet::new("single", vec![(5.0, 0.8)]);
        assert_eq!(set.membership(3.0), 0.8); // below point
        assert_eq!(set.membership(5.0), 0.8); // at point
        assert_eq!(set.membership(7.0), 0.8); // above point
    }

    #[test]
    fn test_membership_below_range() {
        let points = vec![(10.0, 0.0), (20.0, 1.0), (30.0, 0.0)];
        let set = FuzzySet::new("test", points);
        assert_eq!(set.membership(5.0), 0.0); // below minimum
    }

    #[test]
    fn test_membership_above_range() {
        let points = vec![(10.0, 0.0), (20.0, 1.0), (30.0, 0.0)];
        let set = FuzzySet::new("test", points);
        assert_eq!(set.membership(35.0), 0.0); // above maximum
    }

    #[test]
    fn test_membership_at_points() {
        let points = vec![(0.0, 0.0), (5.0, 1.0), (10.0, 0.0)];
        let set = FuzzySet::new("test", points);

        assert_eq!(set.membership(0.0), 0.0);
        assert_eq!(set.membership(5.0), 1.0);
        assert_eq!(set.membership(10.0), 0.0);
    }

    #[test]
    fn test_membership_linear_interpolation() {
        let points = vec![(0.0, 0.0), (10.0, 1.0)];
        let set = FuzzySet::new("test", points);

        assert_eq!(set.membership(5.0), 0.5); // midpoint
        assert_eq!(set.membership(2.5), 0.25); // quarter point
        assert_eq!(set.membership(7.5), 0.75); // three-quarter point
    }

    #[test]
    fn test_membership_triangular_function() {
        let points = vec![(0.0, 0.0), (5.0, 1.0), (10.0, 0.0)];
        let set = FuzzySet::new("test", points);

        assert_eq!(set.membership(2.5), 0.5); // ascending slope
        assert_eq!(set.membership(7.5), 0.5); // descending slope
    }

    #[test]
    fn test_membership_trapezoidal_function() {
        let points = vec![(0.0, 0.0), (2.0, 1.0), (8.0, 1.0), (10.0, 0.0)];
        let set = FuzzySet::new("test", points);

        assert_eq!(set.membership(1.0), 0.5); // ascending
        assert_eq!(set.membership(5.0), 1.0); // plateau
        assert_eq!(set.membership(9.0), 0.5); // descending
    }

    #[test]
    fn test_membership_identical_x_values() {
        let points = vec![(5.0, 0.3), (5.0, 0.7), (10.0, 0.0)];
        let set = FuzzySet::new("test", points);

        // Should return the first y value when x values are identical
        assert_eq!(set.membership(5.0), 0.3);
    }

    // Test FuzzyRule functionality
    #[test]
    fn test_fuzzy_rule_creation() {
        let rule: FuzzyRule<&'static str, &'static str> = FuzzyRule::new("high_temp");
        assert_eq!(rule.condition, "high_temp");
        assert!(rule.consequences.is_empty());
    }

    #[test]
    fn test_fuzzy_rule_with_consequences() {
        let rule: FuzzyRule<&'static str, &'static str> = FuzzyRule::new("high_temp")
            .with_consequence("fan_speed", 0.8)
            .with_consequence("cooling", 0.9);

        assert_eq!(rule.condition, "high_temp");
        assert_eq!(rule.consequences.get("fan_speed"), Some(&0.8));
        assert_eq!(rule.consequences.get("cooling"), Some(&0.9));
    }

    #[test]
    fn test_fuzzy_rule_overwrite_consequence() {
        let rule = FuzzyRule::new("temp")
            .with_consequence("output", 0.5)
            .with_consequence("output", 0.8); // overwrite

        assert_eq!(rule.consequences.get("output"), Some(&0.8));
        assert_eq!(rule.consequences.len(), 1);
    }

    // Test FuzzySystem functionality
    #[test]
    fn test_fuzzy_system_creation() {
        let system: FuzzySystem<&str, &str> = FuzzySystem::new();
        assert!(system.input_sets.is_empty());
        assert!(system.rules.is_empty());
    }

    #[test]
    fn test_fuzzy_system_default() {
        let system: FuzzySystem<&str, &str> = FuzzySystem::default();
        assert!(system.input_sets.is_empty());
        assert!(system.rules.is_empty());
    }

    #[test]
    fn test_add_input_set() {
        let mut system: FuzzySystem<&str, &str> = FuzzySystem::new();
        let set = FuzzySet::new("low", vec![(0.0, 1.0), (50.0, 0.0)]);

        system.add_input_set(set);
        assert_eq!(system.input_sets.len(), 1);
        assert_eq!(system.input_sets[0].category, "low");
    }

    #[test]
    fn test_add_rule() {
        let mut system = FuzzySystem::new();
        let rule = FuzzyRule::new("low").with_consequence("slow", 0.3);

        system.add_rule(rule);
        assert_eq!(system.rules.len(), 1);
        assert_eq!(system.rules[0].condition, "low");
    }

    #[test]
    fn test_evaluate_empty_system() {
        let system: FuzzySystem<&str, &str> = FuzzySystem::new();
        let result = system.evaluate(25.0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_evaluate_no_matching_sets() {
        let mut system = FuzzySystem::new();
        let rule = FuzzyRule::new("nonexistent").with_consequence("output", 1.0);
        system.add_rule(rule);

        let result = system.evaluate(25.0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_evaluate_zero_membership() {
        let mut system = FuzzySystem::new();
        let set = FuzzySet::new("high", vec![(80.0, 0.0), (100.0, 1.0)]);
        let rule = FuzzyRule::new("high").with_consequence("output", 1.0);

        system.add_input_set(set);
        system.add_rule(rule);

        let result = system.evaluate(50.0); // Low input, should have 0 membership
        assert!(result.is_empty());
    }

    #[test]
    fn test_evaluate_single_rule() {
        let mut system = FuzzySystem::new();
        let set = FuzzySet::new("medium", vec![(0.0, 0.0), (50.0, 1.0), (100.0, 0.0)]);
        let rule = FuzzyRule::new("medium").with_consequence("output", 0.7);

        system.add_input_set(set);
        system.add_rule(rule);

        let result = system.evaluate(50.0); // Peak membership
        assert_eq!(result.get("output"), Some(&0.7));
    }

    #[test]
    fn test_evaluate_partial_membership() {
        let mut system = FuzzySystem::new();
        let set = FuzzySet::new("warm", vec![(0.0, 0.0), (50.0, 1.0), (100.0, 0.0)]);
        let rule = FuzzyRule::new("warm").with_consequence("fan", 0.8);

        system.add_input_set(set);
        system.add_rule(rule);

        let result = system.evaluate(25.0); // 0.5 membership
        assert_eq!(result.get("fan"), Some(&0.8));
    }

    #[test]
    fn test_evaluate_multiple_consequences() {
        let mut system = FuzzySystem::new();
        let set = FuzzySet::new("hot", vec![(50.0, 0.0), (100.0, 1.0)]);
        let rule = FuzzyRule::new("hot")
            .with_consequence("fan_speed", 0.9)
            .with_consequence("cooling", 0.8);

        system.add_input_set(set);
        system.add_rule(rule);

        let result = system.evaluate(100.0);
        assert_eq!(result.get("fan_speed"), Some(&0.9));
        assert_eq!(result.get("cooling"), Some(&0.8));
    }

    #[test]
    fn test_evaluate_multiple_rules_same_output() {
        let mut system = FuzzySystem::new();

        // Two overlapping sets
        let low_set = FuzzySet::new("low", vec![(0.0, 1.0), (50.0, 0.0)]);
        let medium_set = FuzzySet::new("medium", vec![(25.0, 0.0), (75.0, 1.0)]);

        // Rules with same output parameter
        let rule1 = FuzzyRule::new("low").with_consequence("action", 0.2);
        let rule2 = FuzzyRule::new("medium").with_consequence("action", 0.8);

        system.add_input_set(low_set);
        system.add_input_set(medium_set);
        system.add_rule(rule1);
        system.add_rule(rule2);

        let result = system.evaluate(37.5);

        // At input 37.5:
        // - "low" membership = 0.25 (25% down from 50 to 0)
        // - "medium" membership = 0.25 (25% up from 25 to 75)
        // Weighted average: (0.2 * 0.25 + 0.8 * 0.25) / (0.25 + 0.25) = 0.5
        assert_eq!(result.get("action"), Some(&0.5));
    }

    #[test]
    fn test_evaluate_complex_system() {
        let mut system = FuzzySystem::new();

        // Temperature sets
        let cold = FuzzySet::new("cold", vec![(0.0, 1.0), (20.0, 0.0)]);
        let warm = FuzzySet::new("warm", vec![(15.0, 0.0), (25.0, 1.0), (35.0, 0.0)]);
        let hot = FuzzySet::new("hot", vec![(30.0, 0.0), (50.0, 1.0)]);

        // Rules
        let rule1 = FuzzyRule::new("cold")
            .with_consequence("heater", 1.0)
            .with_consequence("fan", 0.0);
        let rule2 = FuzzyRule::new("warm")
            .with_consequence("heater", 0.3)
            .with_consequence("fan", 0.3);
        let rule3 = FuzzyRule::new("hot")
            .with_consequence("heater", 0.0)
            .with_consequence("fan", 1.0);

        system.add_input_set(cold);
        system.add_input_set(warm);
        system.add_input_set(hot);
        system.add_rule(rule1);
        system.add_rule(rule2);
        system.add_rule(rule3);

        let result = system.evaluate(25.0); // Peak of "warm"
        assert_eq!(result.get("heater"), Some(&0.3));
        assert_eq!(result.get("fan"), Some(&0.3));
    }

    #[test]
    fn test_evaluate_weighted_average_calculation() {
        let mut system = FuzzySystem::new();

        let set1 = FuzzySet::new("A", vec![(0.0, 0.0), (10.0, 1.0)]);
        let set2 = FuzzySet::new("B", vec![(5.0, 0.0), (15.0, 1.0)]);

        let rule1 = FuzzyRule::new("A").with_consequence("output", 0.2);
        let rule2 = FuzzyRule::new("B").with_consequence("output", 0.8);

        system.add_input_set(set1);
        system.add_input_set(set2);
        system.add_rule(rule1);
        system.add_rule(rule2);

        let result = system.evaluate(10.0);

        // At input 10.0:
        // - A membership = 1.0, contributes 0.2 with weight 1.0
        // - B membership = 0.5, contributes 0.8 with weight 0.5
        // Weighted average: (0.2 * 1.0 + 0.8 * 0.5) / (1.0 + 0.5) = 0.6 / 1.5 = 0.4
        assert!((result.get("output").unwrap() - 0.4).abs() < 1e-6);
    }

    #[test]
    fn test_evaluate_with_numeric_types() {
        let mut system: FuzzySystem<i32, i32> = FuzzySystem::new();

        let set = FuzzySet::new(1, vec![(0.0, 0.0), (100.0, 1.0)]);
        let rule = FuzzyRule::new(1).with_consequence(42, 0.75);

        system.add_input_set(set);
        system.add_rule(rule);

        let result = system.evaluate(50.0);
        assert_eq!(result.get(&42), Some(&0.75));
    }

    #[test]
    fn test_membership_precision() {
        let points = vec![(0.0, 0.0), (1.0, 1.0)];
        let set = FuzzySet::new("test", points);

        // Test very small values
        let membership = set.membership(0.001);
        assert!((membership - 0.001).abs() < 1e-6);

        // Test precision near boundaries
        let membership = set.membership(0.999);
        assert!((membership - 0.999).abs() < 1e-6);
    }

    #[test]
    fn test_system_with_no_rules_matching_input() {
        let mut system = FuzzySystem::new();

        let set1 = FuzzySet::new("low", vec![(0.0, 1.0), (10.0, 0.0)]);
        let set2 = FuzzySet::new("high", vec![(90.0, 0.0), (100.0, 1.0)]);

        // Rule only for "medium" which doesn't exist
        let rule = FuzzyRule::new("medium").with_consequence("output", 1.0);

        system.add_input_set(set1);
        system.add_input_set(set2);
        system.add_rule(rule);

        let result = system.evaluate(50.0);
        assert!(result.is_empty());
    }
}
