defmodule QotWeb.ConnCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint QotWeb.Endpoint

      use Qot.DataCase

      import Plug.Conn
      import Phoenix.ConnTest
      import QotWeb.ConnCase
    end
  end

  setup _tags do
    # DataCase already sets up the sandbox, we just need to add conn
    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end
end
