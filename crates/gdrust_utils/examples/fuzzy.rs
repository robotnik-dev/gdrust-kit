use gdrust_utils::fuzzy::{FuzzyRule, FuzzySet, FuzzySystem};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameParameter {
    EnemyHealth,
    EnemyDamage,
    EnemySpeed,
    ResourceScarcity,
    CheckpointFrequency,
}

pub struct GameDifficulty {
    fuzzy_system: FuzzySystem<DifficultyLevel, GameParameter>,
}

impl Default for GameDifficulty {
    fn default() -> Self {
        Self::new()
    }
}

impl GameDifficulty {
    pub fn new() -> Self {
        let mut system = FuzzySystem::new();

        // Define fuzzy sets
        system.add_input_set(FuzzySet::new(
            DifficultyLevel::Easy,
            vec![(0.0, 1.0), (3.0, 1.0), (5.0, 0.0)],
        ));

        system.add_input_set(FuzzySet::new(
            DifficultyLevel::Normal,
            vec![(3.0, 0.0), (5.0, 1.0), (7.0, 0.0)],
        ));

        system.add_input_set(FuzzySet::new(
            DifficultyLevel::Hard,
            vec![(5.0, 0.0), (7.0, 1.0), (9.0, 0.0)],
        ));

        system.add_input_set(FuzzySet::new(
            DifficultyLevel::Nightmare,
            vec![(7.0, 0.0), (9.0, 1.0), (10.0, 1.0)],
        ));

        // Define rules
        system.add_rule(
            FuzzyRule::new(DifficultyLevel::Easy)
                .with_consequence(GameParameter::EnemyHealth, 0.7)
                .with_consequence(GameParameter::EnemyDamage, 0.7)
                .with_consequence(GameParameter::EnemySpeed, 0.8)
                .with_consequence(GameParameter::ResourceScarcity, 0.2)
                .with_consequence(GameParameter::CheckpointFrequency, 0.9),
        );

        system.add_rule(
            FuzzyRule::new(DifficultyLevel::Normal)
                .with_consequence(GameParameter::EnemyHealth, 1.0)
                .with_consequence(GameParameter::EnemyDamage, 1.0)
                .with_consequence(GameParameter::EnemySpeed, 1.0)
                .with_consequence(GameParameter::ResourceScarcity, 0.5)
                .with_consequence(GameParameter::CheckpointFrequency, 0.6),
        );

        system.add_rule(
            FuzzyRule::new(DifficultyLevel::Hard)
                .with_consequence(GameParameter::EnemyHealth, 1.4)
                .with_consequence(GameParameter::EnemyDamage, 1.3)
                .with_consequence(GameParameter::EnemySpeed, 1.2)
                .with_consequence(GameParameter::ResourceScarcity, 0.8)
                .with_consequence(GameParameter::CheckpointFrequency, 0.3),
        );

        system.add_rule(
            FuzzyRule::new(DifficultyLevel::Nightmare)
                .with_consequence(GameParameter::EnemyHealth, 1.8)
                .with_consequence(GameParameter::EnemyDamage, 1.7)
                .with_consequence(GameParameter::EnemySpeed, 1.5)
                .with_consequence(GameParameter::ResourceScarcity, 0.95)
                .with_consequence(GameParameter::CheckpointFrequency, 0.1),
        );

        Self {
            fuzzy_system: system,
        }
    }

    pub fn get_settings(&self, difficulty_level: f32) -> HashMap<GameParameter, f32> {
        self.fuzzy_system.evaluate(difficulty_level)
    }

    // Convenience method for string-based difficulty
    pub fn get_settings_by_name(&self, difficulty: &str) -> Option<HashMap<GameParameter, f32>> {
        let level = match difficulty.to_lowercase().as_str() {
            "easy" => 2.0,
            "normal" => 5.0,
            "hard" => 7.0,
            "nightmare" => 9.0,
            _ => return None,
        };
        Some(self.get_settings(level))
    }

    pub fn apply_settings(
        &self,
        difficulty_level: f32,
        base_enemy_hp: i32,
        base_damage: i32,
    ) -> GameSettings {
        let settings = self.get_settings(difficulty_level);

        let enemy_health_mult = *settings.get(&GameParameter::EnemyHealth).unwrap_or(&1.0);
        let enemy_damage_mult = *settings.get(&GameParameter::EnemyDamage).unwrap_or(&1.0);

        GameSettings {
            enemy_health_multiplier: enemy_health_mult,
            enemy_damage_multiplier: enemy_damage_mult,
            enemy_speed_multiplier: *settings.get(&GameParameter::EnemySpeed).unwrap_or(&1.0),
            resource_scarcity: *settings
                .get(&GameParameter::ResourceScarcity)
                .unwrap_or(&0.5),
            checkpoint_frequency: *settings
                .get(&GameParameter::CheckpointFrequency)
                .unwrap_or(&0.7),
            actual_enemy_hp: (base_enemy_hp as f32 * enemy_health_mult) as i32,
            actual_enemy_damage: (base_damage as f32 * enemy_damage_mult) as i32,
        }
    }
}

#[derive(Debug)]
pub struct GameSettings {
    pub enemy_health_multiplier: f32,
    pub enemy_damage_multiplier: f32,
    pub enemy_speed_multiplier: f32,
    pub resource_scarcity: f32,
    pub checkpoint_frequency: f32,
    pub actual_enemy_hp: i32,
    pub actual_enemy_damage: i32,
}

fn main() {
    let difficulty_system = GameDifficulty::new();

    println!("=== Fuzzy Logic Example ===");

    let test_values = [1.0, 4.0, 5.0, 6.0, 8.5];

    for &difficulty_val in &test_values {
        println!("\n--- Difficulty Level: {:.1} ---", difficulty_val);

        // Show membership degrees for each difficulty category
        for set in &difficulty_system.fuzzy_system.input_sets {
            let membership = set.membership(difficulty_val);
            if membership > 0.0 {
                println!("  {:?} membership: {:.2}", set.category, membership);
            }
        }

        let settings = difficulty_system.get_settings(difficulty_val);
        println!("  Resulting settings:");
        for (param, value) in &settings {
            println!("    {:?}: {:.3}", param, value);
        }
    }

    println!("\n=== Applied Game Settings ===");
    let base_hp = 100;
    let base_damage = 25;
    println!("\nEnemy base hp: {:.1}", base_hp);
    println!("Enemy base damage: {:.1}", base_damage);

    for &difficulty_val in &[2.0, 5.0, 7.0, 9.0] {
        let settings = difficulty_system.apply_settings(difficulty_val, base_hp, base_damage);
        println!("\nDifficulty {:.1}:", difficulty_val);
        println!(
            "  Enemy HP: {} ({}x)",
            settings.actual_enemy_hp, settings.enemy_health_multiplier
        );
        println!(
            "  Enemy Damage: {} ({}x)",
            settings.actual_enemy_damage, settings.enemy_damage_multiplier
        );
        println!("  Enemy Speed: {}x", settings.enemy_speed_multiplier);
        println!(
            "  Resource Scarcity: {:.0}%",
            settings.resource_scarcity * 100.0
        );
        println!(
            "  Checkpoint Frequency: {:.0}%",
            settings.checkpoint_frequency * 100.0
        );
    }

    // Test string-based interface
    println!("\n=== String Interface Demo (getting Settings by 'name') ===");
    if let Some(settings) = difficulty_system.get_settings_by_name("normal") {
        println!("'Normal' (5.0) difficulty settings:");
        for (param, value) in &settings {
            println!("  {:?}: {:.3}", param, value);
        }
    }
}
