use crate::schema::recipes;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Queryable, Debug, PartialEq, QueryableByName)]
#[table_name = "recipes"]
pub struct Recipe {
  pub id: i32,
  pub title: String,
  pub making_time: String,
  pub serves: String,
  pub ingredients: String,
  pub cost: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct RecipeJson {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i32>,
  pub title: Option<String>,
  pub making_time: Option<String>,
  pub serves: Option<String>,
  pub ingredients: Option<String>,
  #[serde(serialize_with = "serialize_cost")]
  pub cost: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option<NaiveDateTime>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub updated_at: Option<NaiveDateTime>,
}

impl From<&Recipe> for RecipeJson {
  fn from(recipe: &Recipe) -> Self {
    let Recipe {
      id,
      title,
      making_time,
      serves,
      ingredients,
      cost,
      created_at,
      updated_at,
    } = recipe;

    Self {
      id: Some(*id),
      title: Some(title.into()),
      making_time: Some(making_time.into()),
      serves: Some(serves.into()),
      ingredients: Some(ingredients.into()),
      cost: Some(*cost),
      created_at: Some(*created_at),
      updated_at: Some(*updated_at),
    }
  }
}

impl RecipeJson {
  pub fn without_id(self) -> Self {
    Self { id: None, ..self }
  }

  pub fn without_timestamps(self) -> Self {
    Self {
      created_at: None,
      updated_at: None,
      ..self
    }
  }
}

fn serialize_cost<S>(cost_opt: &Option<i32>, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let res = match cost_opt {
    Some(cost) => cost.to_string(),
    None => "".to_string(),
  };
  s.serialize_str(&res)
}

#[derive(Serialize, Debug)]
pub struct GetRecipesResponse {
  pub recipes: Vec<RecipeJson>,
}

#[derive(Deserialize, Debug, Insertable, PartialEq)]
#[table_name = "recipes"]
pub struct PostRecipeRequest {
  pub title: String,
  pub making_time: String,
  pub serves: String,
  pub ingredients: String,
  pub cost: i32,
}

#[derive(Serialize, Debug)]
pub struct PostRecipeResponse {
  pub message: String,
  pub recipe: Vec<RecipeJson>,
}

#[derive(Serialize, Debug)]
pub struct DeleteRecipeResponse {
  pub message: String,
}
