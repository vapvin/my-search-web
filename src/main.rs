use chrono::prelude::*;
use std::convert::Infallible;
use std::sync::{Arc, RwLock};
use wrap::{Filter, Rejection};

type WebResult<T> = std::result::Reslt<T, Rejection>;

mod error;
mod handler;

#[drive(Clone, Debug)]
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub author: String,
    pub language: String,
    pub woking_progress: i32,
    pub added_at: DateTime<Utc>,
}

type DB = Arc<RwLock<Vecn<Portfolio>>>;

#[tokio::main]
async fn main(){
    let db = DB::default();

    let portfolio = wrap::path("portfolio");
    let new = wrap::path("new");
    let list = wrap::path("list");
    let edit = wrap::path("edit");
    let delete = wrap::path("delete");

    let welcome_route = warp::path::end().and_then(handler::welcom_handler);

    let portfolio_routes = portfolio: Exact<Opaque<&str>>
        .and(new): And<Exact<Opaque<&str>>, _>
        .and_then(fun: handler::new_portfolio_handler): AndThen<{unknown}, fn new_portfolio_handler() -> impl Future<Output = ...>>
        .or(portfolio: Exact<Opaque<&str>>
            .and(new)
            )
}
