/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::catchers::Status;
use crate::db::DbConn;
use crate::model::{
    Claims, PlayerSummary, PlayerWithTurns, PlayerWithTurnsAndAdditionalTeam, TeamMerc, TeamPlayer,
};
use crate::sys::SysInfo;
use crate::Error;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;
use urlencoding::FromUrlEncodingError;

/// # Team Roster
/// Get all of the players on a team (returns all players on all teams if no team is provided).
#[openapi(tag = "Players")]
#[get("/players?<team>")]
pub(crate) async fn players(
    team: Option<String>,
    conn: DbConn,
) -> Result<Json<Vec<TeamPlayer>>, Error> {
    match team {
        Some(team) => {
            let parsed_team_name: Result<String, FromUrlEncodingError> = urlencoding::decode(&team);
            match parsed_team_name {
                Ok(team) => {
                    //println!("{}", team);
                    if let Ok(users) = conn.run(|c| TeamPlayer::load(vec![team], c)).await {
                        std::result::Result::Ok(Json(users))
                    } else {
                        Error::not_found()
                    }
                }
                _ => Error::not_found(),
            }
        }
        None => {
            if let Ok(users) = conn.run(|c| TeamPlayer::loadall(c)).await {
                std::result::Result::Ok(Json(users))
            } else {
                Error::not_found()
            }
        }
    }
}

/// # Team Mercenary Roster
/// Get all of the mercenary players on a team (returns all players on all teams if no team is provided).
#[openapi(tag = "Players")]
#[get("/mercs?<team>")]
pub(crate) async fn mercs(team: String, conn: DbConn) -> Result<Json<Vec<TeamMerc>>, Status> {
    let parsed_team_name: Result<String, FromUrlEncodingError> = urlencoding::decode(&team);
    match parsed_team_name {
        Ok(team) => {
            //println!("{}", team);
            if let Ok(users) = conn.run(|c| TeamMerc::load_mercs(vec![team], c)).await {
                std::result::Result::Ok(Json(users))
            } else {
                std::result::Result::Err(Status(rocket::http::Status::NotFound))
            }
        }
        _ => std::result::Result::Err(Status(rocket::http::Status::Conflict)),
    }
}

/// # Me
/// Retrieves all information about currently logged-in user. Should not be accessed by any
/// scraping programs.
#[openapi(skip)]
#[get("/me")]
pub(crate) async fn me(
    cookies: &CookieJar<'_>,
    conn: DbConn,
    config: &State<SysInfo>,
) -> Result<Json<PlayerWithTurnsAndAdditionalTeam>, crate::Error> {
    let c = Claims::from_private_cookie(cookies, config)?;
    let username = c.0.user.clone();
    let user = conn
        .run(move |connection| {
            PlayerWithTurnsAndAdditionalTeam::load(vec![username], false, connection)
        })
        .await
        .ok_or(Error::NotFound {})?;
    if user.name.to_lowercase() == c.0.user.to_lowercase() {
        std::result::Result::Ok(Json(user))
    } else {
        std::result::Result::Err(Error::NotFound {})
    }
}

/// # Player List
/// Returns all players, but provides simplified data structure for smaller payload size. Unlike
/// other methods, this one will return before a player has been part of a roll.
#[openapi(tag = "Players")]
#[get("/players/full")]
pub(crate) async fn player_full(conn: DbConn) -> Result<Json<Vec<PlayerSummary>>, Error> {
    Ok(Json(conn.run(move |c| PlayerSummary::load(c)).await?))
}

/// # Player Batching
/// Batch retrieval of players
#[openapi(tag = "Players")]
#[get("/players/batch?<players>")]
pub(crate) async fn player_multifetch(
    players: Option<String>,
    conn: DbConn,
) -> Result<Json<Vec<PlayerWithTurns>>, Status> {
    match players {
        Some(player) => std::result::Result::Ok(Json(
            conn.run(move |c| {
                PlayerWithTurns::load(
                    player
                        .split(',')
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>(),
                    true,
                    c,
                )
            })
            .await,
        )),
        None => std::result::Result::Err(Status(rocket::http::Status::NotFound)),
    }
}

/// # Player Information
/// Retrieve information about individual player
#[openapi(tag = "Players")]
#[get("/player?<player>")]
pub(crate) async fn player(
    player: String,
    conn: DbConn,
) -> Result<Json<PlayerWithTurnsAndAdditionalTeam>, crate::Error> {
    let users = conn
        .run(|c| PlayerWithTurnsAndAdditionalTeam::load(vec![player], true, c))
        .await.ok_or(crate::Error::NotFound{})?;
        std::result::Result::Ok(Json(users))
}