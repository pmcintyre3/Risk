use crate::catchers::Status;
use crate::db::DbConn;
use crate::model::{Latest, TerritoryHistory, TerritoryTurn, TerritoryWithNeighbors};
use rocket_contrib::json::Json;

#[openapi]
#[get("/territories?<day>&<season>")]
pub async fn territories(
    season: Option<i32>,
    day: Option<i32>,
    conn: DbConn,
) -> Result<Json<Vec<TerritoryWithNeighbors>>, Status> {
    match conn.run(move |c| Latest::latest(c)).await {
        Ok(current) => {
            let territories = conn
                .run(move |c| {
                    TerritoryWithNeighbors::load(
                        season.unwrap_or(current.season),
                        day.unwrap_or(current.day),
                        c,
                    )
                })
                .await;
            if territories.len() as i32 >= 1 {
                std::result::Result::Ok(Json(territories))
            } else {
                std::result::Result::Err(Status(rocket::http::Status::BadRequest))
            }
        }
        _ => std::result::Result::Err(Status(rocket::http::Status::BadRequest)),
    }
}

#[openapi]
#[get("/territory/history?<territory>&<season>")]
pub async fn territoryhistory(
    territory: String,
    season: i32,
    conn: DbConn,
) -> Result<Json<Vec<TerritoryHistory>>, Status> {
    let territories = conn.run(move |c| TerritoryHistory::load(territory, season, c)).await;
    if territories.len() as i32 >= 1 {
        std::result::Result::Ok(Json(territories))
    } else {
        std::result::Result::Err(Status(rocket::http::Status::BadRequest))
    }
}

#[openapi]
#[get("/territory/turn?<territory>&<season>&<day>")]
pub async fn territory_turn(
    territory: String,
    season: i32,
    day: i32,
    conn: DbConn,
) -> Result<Json<TerritoryTurn>, Status> {
    let turn = conn.run(move |c| TerritoryTurn::load(season, day, territory, c)).await;
    match turn {
        Ok(turn) => std::result::Result::Ok(Json(turn)),
        _ => std::result::Result::Err(Status(rocket::http::Status::BadRequest)),
    }
}