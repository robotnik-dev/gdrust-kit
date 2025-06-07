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
    //TODO
}
