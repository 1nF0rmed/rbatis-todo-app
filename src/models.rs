use rbatis::crud_table;
use serde::{Serialize, Deserialize};

#[crud_table]
#[derive(Serialize, Deserialize)]
pub struct Task {
  pub id: Option<String>,
  pub title: Option<String>,
  pub descp: Option<String>,
  pub completed: Option<bool>,
}
