defmodule FerrisExTest do
  use ExUnit.Case
  doctest FerrisEx

  test "greets the world" do
    assert FerrisEx.hello() == :world
  end
end
