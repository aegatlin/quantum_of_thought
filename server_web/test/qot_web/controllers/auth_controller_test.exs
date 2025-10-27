defmodule QotWeb.AuthControllerTest do
  use QotWeb.ConnCase, async: true

  describe "POST /api/auth/magic-link" do
    test "requests a magic link with valid email", %{conn: conn} do
      conn = post(conn, "/api/auth/magic-link", %{email: "test@example.com"})

      assert json_response(conn, 200) == %{"message" => "Magic link sent to test@example.com"}
    end

    test "returns error without email", %{conn: conn} do
      conn = post(conn, "/api/auth/magic-link", %{})

      assert json_response(conn, 400) == %{"error" => "Email is required"}
    end
  end

  describe "GET /api/auth/verify" do
    test "returns error with invalid token", %{conn: conn} do
      conn = get(conn, "/api/auth/verify", %{token: "invalid_token"})

      assert json_response(conn, 401) == %{"error" => "Invalid or expired token"}
    end
  end

  describe "POST /api/auth/refresh" do
    test "returns error with invalid refresh token", %{conn: conn} do
      conn = post(conn, "/api/auth/refresh", %{refresh_token: "invalid"})

      assert json_response(conn, 401) == %{"error" => "Invalid or expired refresh token"}
    end

    test "returns error without refresh token", %{conn: conn} do
      conn = post(conn, "/api/auth/refresh", %{})

      assert json_response(conn, 400) == %{"error" => "Refresh token is required"}
    end
  end

  describe "POST /api/auth/logout" do
    test "returns success message", %{conn: conn} do
      # Logout returns success even with invalid tokens (idempotent)
      conn = post(conn, "/api/auth/logout", %{refresh_token: "any_token"})

      assert json_response(conn, 200) == %{"message" => "Logged out successfully"}
    end

    test "returns error without refresh token", %{conn: conn} do
      conn = post(conn, "/api/auth/logout", %{})

      assert json_response(conn, 400) == %{"error" => "Refresh token is required"}
    end
  end
end
