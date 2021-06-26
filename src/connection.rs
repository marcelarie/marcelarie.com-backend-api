use std::env;
use std::ops::Deref; // <-- TODO: study this more

use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

// Creating a connection pool type that haves a ConnectionManager connected to
// the postgres database
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// take the DATABASE_URL enviorment variable from .env
fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}


pub fn init_pool() -> Pool {
    dotenv().ok(); // <-- handeling dotenv errors
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    // genereting a new ConnectionManager ⤴

    Pool::new(manager).expect("Failed to create Database Pool") // <-- return of the ConnectionManager
}

// Creating a Database Connection struct to return in with the ConnectionManager
// to be used on all the handlers, as a request guard. <-- TODO: study this more
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// Creating the Request Guard from the Connection Pool
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)), // <-- return of the struct
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

