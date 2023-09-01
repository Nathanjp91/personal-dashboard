use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;
use chrono::{DateTime, Utc, Duration};
use ndarray::Array3;

struct Model {
    id: i32,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    parameters: Parameters,
    assets: Vec<Asset>,
    model_type: Box<dyn InvestmentModel>,
    criteria: Box<dyn Criteria>,
    factors: Vec<Box<dyn InvestmentFactors>>,
}
impl Model {
    fn validate(&self) -> Result<(), ModelError> {
        Ok(())
    }
    fn update(&mut self) -> Result<State, ModelError> {
        self.model_type.update(&mut self.assets);
        for factor in &mut self.factors {
            factor.update(&mut self.assets);
        }
        Ok(self.criteria.evaluate(&mut self.assets))
    }
    fn calculate_results(&mut self) -> f64 {
        let mut total = 0.0;
        for asset in &mut self.assets {
            total += asset.calculate_value();
        }
        total
    }
    pub fn run(&mut self) {
        let start_time = Utc::now();
        let mut current_time = Utc::now();
        let mut current_iteration = 0;
        while current_time - start_time < self.parameters.max_time && current_iteration < self.parameters.max_iterations {
            match self.update() {
                Ok(state) => {
                    match state {
                        State::Running => {
                            println!("Model is running");
                        },
                        State::Complete => {
                            println!("Model is complete");
                            break;
                        },
                        State::Failed => {
                            println!("Model has failed due to criteria");
                            break;
                        }
                    }
                },
                Err(e) => {
                    println!("Model has failed due to error: {:?}", e);
                    break;
                }
            }
            current_time = Utc::now();
            current_iteration += 1;
        }
    } 
}
struct Parameters {
    max_iterations: i32,
    max_time: Duration,
}
struct Asset {
    ticker: String,
    amount_held: i32,
    price_history: Option<Vec<f64>>,
}
impl Asset {
    fn calculate_value(&mut self) -> f64 {
        match &self.price_history {
            Some(history) => {
                self.amount_held as f64 * history[history.len() - 1] as f64
            },
            None => 0.0,
        }
    }
    fn latest_price(&mut self) -> f64 {
        match &self.price_history {
            Some(history) => {
                history[history.len() - 1]
            },
            None => 0.0,
        }
    }
}

enum State {
    Running,
    Complete,
    Failed,
}
#[derive(Debug)]
enum ModelError {

}

trait InvestmentFactors {
    fn update(&mut self, assets: &mut Vec<Asset>);
}
trait InvestmentModel {
    fn update(&mut self, assets: &mut Vec<Asset>);
}

trait Criteria {
    fn evaluate(&mut self, assets: &mut Vec<Asset>) -> State;
    fn update_inflation(&mut self, inflation: f64);
}

struct BasicFIRECriteria {
    fire_rate: f64,
    expenses: f64
}
impl Criteria for BasicFIRECriteria {
    fn evaluate(&mut self, assets: &mut Vec<Asset>) -> State {
        let total_value = assets.into_iter().fold(0.0, |acc, asset| acc + asset.calculate_value());
        match total_value * self.fire_rate > self.expenses {
            true => State::Complete,
            false => State::Running,
        }
    }
    fn update_inflation(&mut self, inflation: f64) {
        self.expenses *= 1.0 + inflation;
    }
}

struct MonthlyInvestmentFactor {
    leftover: f64,
    monthly_investment: f64,
    last_investment: DateTime<Utc>,
    desired_allocation: HashMap<String, f64>,
}

impl InvestmentFactors for MonthlyInvestmentFactor {
    fn update(&mut self, assets: &mut Vec<Asset>) {
        let to_spend = self.monthly_investment + self.leftover;
        let mut spent = 0.0;
        let rng = &mut thread_rng();
        assets.shuffle(rng); // Shuffle the assets to avoid bias in the order of buying
        for asset in assets {
            let allocation = self.desired_allocation.get(&asset.ticker);
            match allocation {
                Some(allocation) => {
                    let price = asset.latest_price();
                    let amount_to_spend = to_spend * allocation;
                    let amount_to_buy = (amount_to_spend / price) as i32;
                    spent += amount_to_buy as f64 * price;
                    asset.amount_held += amount_to_buy;
                },
                None => {
                    println!("No allocation for asset {}", asset.ticker);
                }
            }
        }
        self.leftover = to_spend - spent;
    }
}

struct MarkovChainModel {
    transition_matrix: Array3<f64>,
    current_state: usize,
}

impl InvestmentModel for MarkovChainModel {
    fn update(&mut self, assets: &mut Vec<Asset>) {
        
    }
}