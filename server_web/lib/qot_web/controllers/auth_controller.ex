defmodule QotWeb.AuthController do
  use QotWeb, :controller

  alias Qot.Accounts

  @doc """
  POST /api/auth/magic-link
  Request a magic link to be sent to the given email.
  Body: {"email": "user@example.com"}
  """
  def request_magic_link(conn, %{"email" => email}) do
    case Accounts.create_magic_link(email) do
      :ok ->
        json(conn, %{message: "Magic link sent to #{email}"})

      {:error, _reason} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: "Failed to send magic link"})
    end
  end

  def request_magic_link(conn, _params) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "Email is required"})
  end

  @doc """
  GET /api/auth/verify?token=xyz
  Verify a magic link token and return access + refresh tokens.
  """
  def verify_magic_link(conn, %{"token" => token}) do
    case Accounts.verify_magic_link(token) do
      {:ok, %{access_token: access_token, refresh_token: refresh_token, user: user}} ->
        json(conn, %{
          access_token: access_token,
          refresh_token: refresh_token,
          user: %{
            id: user.id,
            email: user.email
          }
        })

      {:error, :invalid_or_expired_token} ->
        conn
        |> put_status(:unauthorized)
        |> json(%{error: "Invalid or expired token"})

      {:error, _reason} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: "Failed to verify token"})
    end
  end

  def verify_magic_link(conn, _params) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "Token is required"})
  end

  @doc """
  POST /api/auth/refresh
  Refresh an access token using a refresh token.
  Body: {"refresh_token": "abc..."}
  """
  def refresh(conn, %{"refresh_token" => refresh_token}) do
    case Accounts.refresh_access_token(refresh_token) do
      {:ok, %{access_token: access_token}} ->
        json(conn, %{access_token: access_token})

      {:error, :invalid_or_expired_token} ->
        conn
        |> put_status(:unauthorized)
        |> json(%{error: "Invalid or expired refresh token"})

      {:error, _reason} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: "Failed to refresh token"})
    end
  end

  def refresh(conn, _params) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "Refresh token is required"})
  end

  @doc """
  POST /api/auth/logout
  Revoke a refresh token (logout).
  Body: {"refresh_token": "abc..."}
  """
  def logout(conn, %{"refresh_token" => refresh_token}) do
    case Accounts.revoke_refresh_token(refresh_token) do
      :ok ->
        json(conn, %{message: "Logged out successfully"})

      {:error, _reason} ->
        conn
        |> put_status(:internal_server_error)
        |> json(%{error: "Failed to logout"})
    end
  end

  def logout(conn, _params) do
    conn
    |> put_status(:bad_request)
    |> json(%{error: "Refresh token is required"})
  end
end
