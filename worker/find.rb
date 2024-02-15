require "redis"
require_relative "anilist"
require_relative "add_show"

$redis = Redis.new(url: ENV.fetch("REDIS_URL", "redis://localhost:6379"))

def print_show(show, added)
  title = show["title"]["english"] || show["title"]["native"]
  puts "[#{added ? "✅" : "❌"}] [#{show["id"]}] Episode #{show["nextAiringEpisode"]["episode"]} - #{title}"
end

shows = query_shows_airing_now
shows_state = {added: [], others: []}

shows.each do |show|
  value = $redis.get("show:#{show["id"]}")
  if show["nextAiringEpisode"]
    if value
      shows_state[:added] << show
    else
      shows_state[:others] << show
    end
  end
end

puts "Currently watching shows:"
shows_state[:added].each do |show|
  print_show(show, true)
end

puts "Other shows:"
shows_state[:others].each do |show|
  print_show(show, false)
end
