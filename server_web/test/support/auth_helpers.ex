defmodule QotWeb.AuthHelpers do
  @moduledoc """
  Test helpers for authentication.
  """

  alias Qot.Accounts

  @doc """
  Creates a test user and returns their access token and user_id.
  """
  def create_authenticated_user(email \\ "test@example.com") do
    # Create user (SQL Sandbox will handle cleanup)
    {:ok, user} = Accounts.create_user(%{email: email})

    # Generate access token
    {:ok, access_token, _claims} = Accounts.Token.generate_jwt(user.id)

    %{user_id: user.id, access_token: access_token, email: email}
  end

  @doc """
  Adds authentication header to a conn.
  """
  def authenticate_conn(conn, access_token) do
    Plug.Conn.put_req_header(conn, "authorization", "Bearer #{access_token}")
  end
end
