use anyhow::{Context, Result};
use serde::Deserialize;
use strum::Display;

static HR_CHALLENGES: &str =
  "https://www.hackerrank.com/rest/contests/master/tracks/sql/challenges";
static CHALLENGE_PREFIX: &str = "https://www.hackerrank.com/challenges/";
static CHALLENGE_SUFFIX: &str = "/problem?isFullScreen=true";

#[derive(Debug)]
struct ChallengeSettings {
  offset: usize,
  limit: usize,
  track_login: bool,
  filters: Filters,
}

impl ChallengeSettings {
  fn to_query(&self) -> String {
    let mut base = format!(
      "offset={}&limit={}&track_login={}",
      self.offset, self.limit, self.track_login
    );

    let Filters {
      status,
      difficulty,
      subdomains,
      skills,
    } = &self.filters;

    for status in status {
      base.push_str("&filters[status][]=");
      base.push_str(&status.to_string());
    }
    for difficulty in difficulty {
      base.push_str("&filters[difficulty][]=");
      base.push_str(&difficulty.to_string());
    }
    for subdomains in subdomains {
      base.push_str("&filters[subdomains][]=");
      base.push_str(&subdomains.to_string());
    }
    for skills in skills {
      base.push_str("&filters[skills][]=");
      base.push_str(&skills.to_string());
    }

    base
  }
}

#[derive(Debug, Default)]
struct Filters {
  status: Vec<StatusFilter>,
  difficulty: Vec<DifficultyFilter>,
  subdomains: Vec<SubdomainsFilter>,
  skills: Vec<SkillsFilter>,
}

#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
enum StatusFilter {
  Solved,
  Unsolved,
}

#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
enum DifficultyFilter {
  Easy,
  Medium,
  Hard,
}

#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
enum SubdomainsFilter {
  Select,
  AdvancedSelect,
  Aggregation,
  Join,
  AdvancedJoin,
}

#[derive(Debug, Display)]
enum SkillsFilter {
  #[strum(serialize = "SQL (Basic)")]
  Basic,
  #[strum(serialize = "SQL (Intermediate)")]
  Intermediate,
  #[strum(serialize = "SQL (Advanced)")]
  Advanced,
}

impl ChallengeSettings {
  pub fn all() -> Self {
    ChallengeSettings {
      offset: 0,
      limit: 100,
      track_login: true,
      filters: Filters {
        status: vec![StatusFilter::Solved, StatusFilter::Unsolved],
        difficulty: vec![
          DifficultyFilter::Easy,
          DifficultyFilter::Medium,
          DifficultyFilter::Hard,
        ],
        subdomains: vec![
          SubdomainsFilter::Select,
          SubdomainsFilter::AdvancedSelect,
          SubdomainsFilter::Aggregation,
          SubdomainsFilter::Join,
          SubdomainsFilter::AdvancedJoin,
        ],
        skills: vec![
          SkillsFilter::Basic,
          SkillsFilter::Intermediate,
          SkillsFilter::Advanced,
        ],
      },
    }
  }

  pub fn no_filters() -> Self {
    ChallengeSettings {
      offset: 0,
      limit: 100,
      track_login: true,
      filters: Filters::default(),
    }
  }

  pub fn easy() -> Self {
    ChallengeSettings {
      offset: 0,
      limit: 100,
      track_login: true,
      filters: Filters {
        difficulty: vec![DifficultyFilter::Easy],
        ..Default::default()
      },
    }
  }
}

#[derive(Debug, Deserialize)]
struct HRResponse {
  models: Vec<Challenge>,
}

#[derive(Debug, Deserialize)]
pub struct Challenge {
  slug: String,
}

impl Challenge {
  pub fn link(&self) -> String {
    let slug = &self.slug;
    format!("{CHALLENGE_PREFIX}{slug}{CHALLENGE_SUFFIX}")
  }

  pub fn fetch_no_filters() -> Result<Vec<Self>> {
    Self::fetch(&ChallengeSettings::no_filters())
  }

  pub fn fetch_all() -> Result<Vec<Self>> {
    Self::fetch(&ChallengeSettings::all())
  }

  pub fn fetch_easy() -> Result<Vec<Self>> {
    Self::fetch(&ChallengeSettings::easy())
  }

  fn fetch(settings: &ChallengeSettings) -> Result<Vec<Self>> {
    let url = format!("{HR_CHALLENGES}?{}", settings.to_query());
    let client = reqwest::blocking::Client::new();
    let response: HRResponse = client
      .get(url)
      .header("User-Agent", "curl/7.81.0")
      .send()
      .context("Failed sending HR request")?
      .json()
      .context("Failed parsing HR response")?;

    Ok(response.models)
  }
}
