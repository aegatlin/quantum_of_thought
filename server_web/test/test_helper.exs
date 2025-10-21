ExUnit.start()

# Start the application so PubSub and other services are available
{:ok, _} = Application.ensure_all_started(:qot)
