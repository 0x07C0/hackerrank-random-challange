use hackerrank_random_challange::api::Challenge;
use rand::seq::SliceRandom;

fn main() -> anyhow::Result<()> {
  let mut rng = rand::thread_rng();

  let challenges = Challenge::fetch_all()?;
  let to_solve = challenges.as_slice().choose_multiple(&mut rng, 3);

  for (i, challenge) in to_solve.enumerate() {
    println!("Challenge {}: {}", i + 1, challenge.link());
  }

  let start = std::time::Instant::now();

  let _ = std::io::stdin().lines().next();

  println!("Total time: {:?}", start.elapsed());

  Ok(())
}
