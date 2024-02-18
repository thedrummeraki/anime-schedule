use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    anilist::types::{FindMultipleShowsVariables, GraphqlRequestBody},
    cli::CliResult,
};

use super::types::{GraphqlResponse, Season, Show, ShowStatus};

pub struct FindShowsOptions {
    pub season: Option<Season>,
    pub year: Option<i32>,
    pub show_status: ShowStatus,
}

pub async fn find_shows(options: &FindShowsOptions) -> CliResult<Vec<Show>> {
    let client = reqwest::Client::new();
    let query = include_str!("./query-multiple.graphql").to_string();

    let mut shows: Vec<Show> = Vec::new();
    let mut has_next_page = true;
    let mut page = 1;

    while has_next_page {
        let variables = FindMultipleShowsVariables {
            page,
            season: options.season.clone(),
            sort: "POPULARITY_DESC".into(),
            show_type: "ANIME".into(),
            status: "RELEASING".into(),
            season_year: options.year,
        };

        let body: GraphqlRequestBody<FindMultipleShowsVariables> = GraphqlRequestBody {
            query: query.clone(),
            variables: Some(variables),
        };

        let request = client
            .post("https://graphql.anilist.co")
            .json(&body)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json");

        let response = request
            .send()
            .await
            .unwrap()
            .json::<GraphqlResponse<Show>>()
            .await
            .unwrap();

        match response.data.page {
            Some(graphql_page) => {
                shows.extend(graphql_page.media);
                has_next_page = graphql_page.page_info.hax_next_page;
                page += 1;
            }
            None => {
                has_next_page = false;
            }
        }
    }

    Ok(shows)
}
