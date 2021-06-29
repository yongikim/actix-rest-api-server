use crate::models;
use crate::schema::recipes::dsl;
use crate::AppState;
use actix_web;
use actix_web::error;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use derive_more::{Display, Error};
use diesel;
use diesel::prelude::*;
use std::sync::Mutex;

#[patch("/recipes/{id}")]
async fn patch_recipe(
    path: web::Path<i32>,
    request_json: web::Json<models::PostRecipeRequest>,
    data: web::Data<Mutex<AppState>>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::*;
    let conn = &data.lock().unwrap().pool;

    let models::PostRecipeRequest {
        title: new_title,
        making_time: new_making_time,
        serves: new_serves,
        ingredients: new_ingredients,
        cost: new_cost,
    } = request_json.into_inner();

    diesel::update(recipes.filter(id.eq(path.0)))
        .set((
            title.eq(new_title.unwrap()),
            making_time.eq(new_making_time.unwrap()),
            serves.eq(new_serves.unwrap()),
            ingredients.eq(new_ingredients.unwrap()),
            cost.eq(new_cost.unwrap()),
        ))
        .execute(conn)
        .unwrap();

    let response = recipes.find(path.0).load::<models::Recipe>(conn).unwrap();

    let recipe = response
        .iter()
        .map(|r| {
            models::RecipeJson::from(r)
                .without_id()
                .without_timestamps()
        })
        .collect();

    let message = "Recipe successfully updated!".to_string();
    let response = models::PostRecipeResponse { message, recipe };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Display, Error)]
#[display(fmt = "message: {}, required: {}", message, required)]
struct PostRequestError {
    message: &'static str,
    required: &'static str,
}

impl Default for PostRequestError {
    fn default() -> Self {
        Self {
            message: &"Recipe creation failed!",
            required: &"tltle, making_time, serves, ingredients, cost",
        }
    }
}

impl error::ResponseError for PostRequestError {}

#[post("/recipes")]
async fn post_recipe(
    request_json: web::Json<models::PostRecipeRequest>,
    data: web::Data<Mutex<AppState>>,
) -> actix_web::Result<impl Responder, HttpResponse> {
    println!("hello");
    use crate::schema::recipes::dsl::*;
    let conn = &data.lock().unwrap().pool;
    let request = request_json.into_inner();

    let result: std::result::Result<models::Recipe, _> = diesel::insert_into(recipes)
        .values(request)
        .get_result(conn);

    match result {
        Ok(res) => {
            let recipe = vec![models::RecipeJson::from(&res)];
            let message = "Recipe successfully created!".to_string();
            let response = models::PostRecipeResponse { message, recipe };

            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => {
            let s = r#"{"message":"Recipe creation failed!","required":"tltle, making_time, serves, ingredients, cost"}"#;
            Err(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(s))
        }
    }
}

#[get("/recipes")]
async fn get_recipes(data: web::Data<Mutex<AppState>>) -> actix_web::Result<impl Responder> {
    let conn = &data.lock().unwrap().pool;
    let result = dsl::recipes
        .limit(5)
        .load::<models::Recipe>(conn)
        .expect("Error loading recipes");

    let recipes: Vec<models::RecipeJson> = result
        .iter()
        .map(|recipe| models::RecipeJson::from(recipe).without_timestamps())
        .collect();

    let response = models::GetRecipesResponse { recipes };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/recipes/{id}")]
async fn get_recipe(
    path: web::Path<i32>,
    data: web::Data<Mutex<AppState>>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::recipes;
    let conn = &data.lock().unwrap().pool;

    let response = recipes.find(path.0).load::<models::Recipe>(conn).unwrap();

    let recipe = response
        .iter()
        .map(|r| models::RecipeJson::from(r).without_timestamps())
        .collect();

    let message = "Recipe details by id".to_string();
    let response = models::PostRecipeResponse { message, recipe };

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/recipes/{id}")]
async fn delete_recipe(
    path: web::Path<i32>,
    data: web::Data<Mutex<AppState>>,
) -> actix_web::Result<impl Responder> {
    use crate::schema::recipes::dsl::{id, recipes};
    let conn = &data.lock().unwrap().pool;

    diesel::delete(recipes.filter(id.eq(path.0)))
        .execute(conn)
        .unwrap();

    let message = "Recipe successfully removed!".to_string();
    let response = models::DeleteRecipeResponse { message };

    Ok(HttpResponse::Ok().json(response))
}
