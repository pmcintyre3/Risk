/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::catchers::Status;
use crate::db::DbConn;
use crate::model::{TeamInfo, TeamPlayerMoves, TerritoryHistory};
use rocket::serde::json::Json;

/// # List of Teams
/// Lists all teams, including those from past seasons.
#[openapi(tag = "Teams", ignore = "conn")]
#[get("/teams")]
pub(crate) async fn teams(conn: DbConn) -> Result<Json<Vec<TeamInfo>>, Status> {
    let teams = conn.run(move |c| TeamInfo::load(c)).await;
    if teams.len() as i32 >= 1 {
        std::result::Result::Ok(Json(teams))
    } else {
        std::result::Result::Err(Status(rocket::http::Status::NotFound))
    }
}

/// # Team Moves
/// List of all moves made by all players on a team on a provided day.
#[openapi(tag = "Teams", ignore = "conn")]
#[get("/team/players?<season>&<day>&<team>")]
pub(crate) async fn teamplayersbymoves(
    season: i32,
    day: i32,
    team: Option<String>,
    conn: DbConn,
) -> Result<Json<Vec<TeamPlayerMoves>>, Status> {
    if let Ok(moves) = conn
        .run(move |c| TeamPlayerMoves::load(season, day, team, c))
        .await
    {
        std::result::Result::Ok(Json(moves))
    } else {
        std::result::Result::Err(Status(rocket::http::Status::NotFound))
    }
}

/// # Season-Visited Map
/// List of all territories visited by a team during a season.
#[openapi(tag = "Teams", ignore = "conn")]
#[get("/team/territories_visited?<season>&<team>")]
pub(crate) async fn team_territories_visited_by_season(
    season: i32,
    team: String,
    conn: DbConn,
) -> Result<Json<Vec<TerritoryHistory>>, Status> {
    if let Ok(moves) = conn
        .run(move |c| TerritoryHistory::load_by_team_in_season(team, season, c))
        .await
    {
        std::result::Result::Ok(Json(moves))
    } else {
        std::result::Result::Err(Status(rocket::http::Status::NotFound))
    }
}
