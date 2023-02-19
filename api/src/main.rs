use demo_triangulation_core as tri;
use axum::{extract::Form, response::{Html}, response::Json, routing::{get, post}, Router};
use axum_extra::{routing::SpaRouter};
use serde::Deserialize;
use std::net::SocketAddr;


async fn sup() -> Html<&'static str> {
  Html(
    r#"
    <!doctype html>
    <html>
      <head>
      </head>
      <body>
        <h1>Sup</h1>
      </body>
    </html>
    "#
  )
}

#[derive(Deserialize, Debug)]
struct Input {
  p1x: i64,
  p1y: i64,
  p2x: i64,
  p2y: i64,
  p3x: i64,
  p3y: i64,
  p4x: i64,
  p4y: i64
}

impl Input {

  async fn to_lines(&self) -> (tri::Line, tri::Line) {
    let l1 = tri::Line::new(
      tri::Point {x: self.p1x, y: self.p1y},
      tri::Point {x: self.p2x, y: self.p2y}
    ).unwrap();

    let l2 = tri::Line::new(
      tri::Point {x: self.p3x, y: self.p3y},
      tri::Point {x: self.p4x, y: self.p4y}
    ).unwrap();

    return (l1, l2)
  }

}

async fn accept_form(Form(input): Form<Input>) -> Json<tri::Point> {
  let (l1, l2) = input.to_lines().await;

  let pi = tri::intersect(&l1, &l2).unwrap();

  return Json(pi);
}

#[tokio::main]
async fn main() {
  let router = Router::new()
    .route("/submit", post(accept_form))
    .route("/sup", get(sup))
    .merge(SpaRouter::new("/", "assets"));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}
