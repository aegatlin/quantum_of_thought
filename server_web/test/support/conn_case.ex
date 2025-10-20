defmodule QotWeb.ConnCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint QotWeb.Endpoint

      import Plug.Conn
      import Phoenix.ConnTest
      import QotWeb.ConnCase
    end
  end

  setup _tags do
    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end
end
