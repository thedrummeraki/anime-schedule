require "redis"
require_relative "anilist"
require_relative "calendar"
require_relative "add_show"

calendar = Calendar.new(ENV.fetch("CALENDAR_ID", "<calendar-id>"))
$redis = Redis.new(url: ENV.fetch("REDIS_URL", "redis://localhost:6379"))

$redis.keys("show:*").each do |key|
  show_id = key.split(":").last

  add_show_to_calendar_if_absent(show_id, calendar)
end
