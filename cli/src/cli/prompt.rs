use crate::anilist::types::Show;

pub fn choose_shows(shows: &[Show]) -> Vec<Show> {
    let result = inquire::MultiSelect::new(
        "Select shows that you want to keep track of:",
        shows.to_vec(),
    )
    .prompt()
    .unwrap();

    Vec::from(result)
}
