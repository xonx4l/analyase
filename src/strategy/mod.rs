pub mod mean_reversion;

#[derive(Debug, Clone)]
pub enum StrategyControl {
    Start,
    Stop,
    UpdateParams(StrategyConfig),
}