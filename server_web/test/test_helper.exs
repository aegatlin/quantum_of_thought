ExUnit.start()

# Set up Ecto SQL Sandbox for tests
Ecto.Adapters.SQL.Sandbox.mode(Qot.Repo, :manual)

# Start the application so PubSub and other services are available
{:ok, _} = Application.ensure_all_started(:qot)
