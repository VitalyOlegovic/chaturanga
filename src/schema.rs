use juniper::FieldResult;
use juniper::RootNode;
use chrono::{NaiveDate,Utc};

use super::types::*;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(build_human(
            String::from("1234"),
            "Luke".to_owned(),
            vec![Episode::NewHope],
            "Mars".to_owned(),
        ))
    }

    fn game(id: String) -> FieldResult<ChessGame> {
        Ok(build_chess_game(
            String::from("1"),
            String::from("Magnus Carlsen"),
            String::from("Fabiano Caruana"),
            Utc::now().naive_local().date()
        ))
    }

}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        Ok(build_human (
            String::from("1234"),
            new_human.name,
            new_human.appears_in,
            new_human.home_planet,
        ))
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
