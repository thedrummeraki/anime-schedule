use clap_derive::Args;

use crate::{
    anilist::{
        api::{self, FindShowsOptions},
        types::{Season, ShowStatus},
    },
    cli::prompt,
};

use super::CliResult;

#[derive(Args)]
pub struct Find {
    #[arg(short, long)]
    pub season: Option<Season>,

    #[arg(short, long)]
    pub year: Option<i32>,

    #[arg(long, conflicts_with_all = &["season", "year"])]
    pub now: bool,
}

pub async fn execute(find: &Find) -> CliResult<()> {
    let (season, year) = if find.now {
        let (season, year) = current_season_year();
        (Some(season), Some(year))
    } else {
        (find.season.clone(), find.year)
    };

    let options = FindShowsOptions {
        season,
        year,
        show_status: ShowStatus::Airing,
    };

    let shows = api::find_shows(&options).await?;
    let mut shows = shows
        .into_iter()
        .filter(|show| show.next_airing_episode.is_some())
        .collect::<Vec<_>>();

    if shows.len() > 0 {
        println!("Found {} show(s)", shows.len());
        shows = prompt::choose_shows(&shows);
    }

    if shows.len() > 0 {
        println!("Choosing the following shows:");
        for show in shows {
            println!("\t{show}")
        }
    }

    Ok(())
}

fn current_season_year() -> (Season, i32) {
    (Season::Winter, 2024)
}
