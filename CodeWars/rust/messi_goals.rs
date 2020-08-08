// https://www.codewars.com/kata/55ca77fa094a2af31f00002a/rust

fn messi_goals() -> u32 {
  static la_liga_goals: u32 = 43;
  static champions_league_goals: u32 = 10;
  static copa_del_rey_goals: u32 = 5;

  (la_liga_goals + champions_league_goals + copa_del_rey_goals)
}

