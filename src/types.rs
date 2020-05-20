use chrono::{NaiveDate};

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub fn build_human(id: String, name: String,  appears_in: Vec<Episode>, home_planet: String) -> Human {
  Human{
    id,
    name,
    appears_in,
    home_planet
  }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A chess game")]
pub struct ChessGame {
  id: String,
  player_white: String,
  player_black: String,
  date: NaiveDate
}

pub fn build_chess_game(  id: String, player_white: String, player_black: String, date: NaiveDate) -> ChessGame {
  ChessGame{id, player_white, player_black, date}
}