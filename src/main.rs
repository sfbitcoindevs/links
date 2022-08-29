use {
  chrono::naive::NaiveDate,
  serde::Deserialize,
  std::{collections::BTreeSet, fs},
  url::Url,
};

#[derive(Deserialize)]
#[allow(unused)]
struct Link {
  title: String,
  url: Url,
  tags: BTreeSet<Tag>,
  date: NaiveDate,
  summary: String,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Tag {
  Lightning,
  Research,
  Paper,
  Security,
}

fn main() {
  let yaml = fs::read_to_string("links.yaml").unwrap();
  serde_yaml::from_str::<Vec<Link>>(&yaml).unwrap();
}
