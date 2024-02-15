require "google/apis/calendar_v3"
require "googleauth"

class Calendar
  def initialize(id)
    @service = Google::Apis::CalendarV3::CalendarService.new
    @service.authorization = credentials
    @service.authorization.fetch_access_token!

    @calendar_id = id
  end

  def list
    @service.list_calendar_lists
  end

  def events
    @service.list_events(@calendar_id)
  end

  def get
    return @calendar if @calendar
    @calendar = @service.get_calendar(@calendar_id)
  rescue Google::Apis::ClientError => e
    puts "Error finding calendar by id `#{@calendar_id}`: #{e.message} (#{e.class})"
    nil
  end

  def insert_event(event)
    @service.insert_event(@calendar_id, event)
  end

  private

  def credentials
    return @credentials if @credentials

    filename = ENV.fetch("GOOGLE_APPLICATION_CREDENTIALS", "service-account-key.json")
    @credentials = Google::Auth::ServiceAccountCredentials.make_creds(
      json_key_io: File.open(filename),
      scope: "https://www.googleapis.com/auth/calendar"
    )
    end
end
