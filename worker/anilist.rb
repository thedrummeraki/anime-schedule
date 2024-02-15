require "rest-client"
require "json"

ONE_MONTH_DURATION = 60 * 60 * 24 * 30.4167

def query_shows_airing_now
  puts "Fetching airing shows..."
  dates = [Time.now - (8 * ONE_MONTH_DURATION), Time.now - (4 * ONE_MONTH_DURATION), Time.now + (4 * ONE_MONTH_DURATION), Time.now]
  shows = []
  dates.map do |date|
    shows += airing_shows_in(date)
  end

  shows.uniq do |show|
    show["id"]
  end
end

def airing_shows_in(date = nil)
  shows = []
  has_next_page = true
  page = 1

  while has_next_page
    variables = generate_variables(page: page, date: date)
    payload = {"query": graphql_query, "variables": variables}.to_json
    response = RestClient.post("https://graphql.anilist.co", payload, {content_type: :json, accept: :json})
    data = JSON.parse(response.body)["data"]["Page"]
    page = data["pageInfo"]["currentPage"] + 1
    has_next_page = data["pageInfo"]["hasNextPage"]

    shows += data["media"]

    sleep(1)
  end

  shows
end

def graphql_query
  File.read("./query-multiple.graphql")
end

def generate_variables(page: 1, date: nil)
  {
    "page": page,
    "sort": "POPULARITY_DESC",
    "status": "RELEASING",
    "type": "ANIME"
  }.merge(season_info_from(date))
end

def season_info_from(date)
  date = Time.now unless date
  year = date.year
  month = date.month
  season = case month
  when 1..3
    "WINTER"
  when 4..6
    "SPRING"
  when 7..9
    "SUMMER"
  when 10..12
    "FALL"
  end
  
  {"season": season, "seasonYear": year}
end
