use anyhow::Context;
use rand::seq::SliceRandom;
use serde::Deserialize;

static HR_CHALLENGES: &str = "https://www.hackerrank.com/rest/contests/master/tracks/sql/challenges?offset=0&limit=100&track_login=true";
static CHALLENGE_PREFIX: &str = "https://www.hackerrank.com/challenges/";
static CHALLENGE_SUFFIX: &str = "/problem?isFullScreen=true";

#[derive(Debug, Deserialize)]
struct HRResponse {
  models: Vec<Challenge>,
}

#[derive(Debug, Deserialize)]
struct Challenge {
  slug: String,
}

impl Challenge {
  fn link(&self) -> String {
    let slug = &self.slug;
    format!("{CHALLENGE_PREFIX}{slug}{CHALLENGE_SUFFIX}")
  }
}

fn main() -> anyhow::Result<()> {
  let client = reqwest::blocking::Client::new();
  let response: HRResponse = client
    .get(HR_CHALLENGES)
    .header("User-Agent", "curl/7.81.0")
    .send()
    .context("Failed sending HR request")?
    .json()
    .context("Failed parsing HR response")?;

  let mut rng = rand::thread_rng();

  let to_solve = response.models.as_slice().choose_multiple(&mut rng, 3);

  for (i, challenge) in to_solve.enumerate() {
    println!("Challenge {}: {}", i + 1, challenge.link());
  }

  let start = std::time::Instant::now();

  let _ = std::io::stdin().lines().next();

  println!("Total time: {:?}", start.elapsed());

  Ok(())
}
