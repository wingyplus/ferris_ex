defmodule FerrisEx.Native do
  use Rustler, otp_app: :ferris_ex, crate: :ferrisex_native

  @doc """
  Print out Ferris saying something.

  Example:

      iex> FerrisEx.Native.say("Hello, Ferris") |> elem(1) |> IO.puts()
  """
  def say(_text), do: :erlang.nif_error(:nif_not_loaded)
end
