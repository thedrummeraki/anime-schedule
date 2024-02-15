require "rest-client"
require "redis"
require_relative "calendar"
require_relative "anilist"

calendar_id = ENV.fetch("CALENDAR_ID", "<calendar-id>")
calendar = Calendar.new(calendar_id)

$redis = Redis.new(url: ENV.fetch("REDIS_URL", "redis://localhost:6379"))

if __FILE__ == $0 && !calendar.get
  puts "Calendar not found. Exiting..."
  exit 1
end

if __FILE__ == $0 && ARGV.length < 1
  puts "Usage: ruby add_show.rb <id...>"
  exit 0
end

def get_show(id)
  query = File.read("./query-one.graphql")
  variables = {"id": id, "type": "ANIME", "isAdult": false}

  response = RestClient.post("https://graphql.anilist.co", {"query": query, "variables": variables}.to_json, {content_type: :json, accept: :json})
  JSON.parse(response.body)["data"]["Media"]
rescue RestClient::Exception => e
  puts e.response
  nil
end

def add_show_to_calendar_if_absent(show_id, calendar)
  if calendar.get.nil?
    puts "Calendar not found. Exiting..."
    return
  end

  show = get_show(show_id)

  if show.nil?
    puts "Show not found by id #{show_id}"
    exit 1
  end

  show_title = show["title"]["english"] || show["title"]["native"]
  puts "Adding `#{show_title}' to calendar `#{calendar.get.summary}'..."

  if show["status"] != "RELEASING"
    puts "Show `#{show_title}' is not currently airing. Skipping..."
    return
  end

  next_episode = show["nextAiringEpisode"]["episode"]
  event_timestamp = show["nextAiringEpisode"]["airingAt"]

  description = "Next episode: #{next_episode}"
  calendar_event = Google::Apis::CalendarV3::Event.new(
    summary: show_title,
    description: description,
    start: {
      date_time: Time.at(event_timestamp).to_datetime.rfc3339
    },
    end: {
      date_time: Time.at(event_timestamp + 60 * 30).to_datetime.rfc3339
    }
  )

  unless $redis.set("show:#{show_id}", next_episode) == "OK"
    puts "Warning: failed to set show `#{show_title}' in Redis."
  end

  existing_events = calendar.events.items.select { |item| item.summary == show_title && item.description == description }
  if existing_events.any?
    puts "Event already exists for `#{show_title}'. Skipping..."
  else
    calendar.insert_event(calendar_event)
    puts "✅ Event added for `#{show_title}'"
  end
end

if __FILE__ == $0
  ARGV.each do |id|
    add_show_to_calendar_if_absent(id, calendar)
  end
end
