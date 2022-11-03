defmodule ValueTest do
  use ExUnit.Case

  describe "Value.get" do
    test "get required value in plain key atom map" do
      scope = %{a: 1, b: 2, c: 3}
      assert 1 == Value.getr!(scope, "a")
      assert {:ok, 1} == Value.getr(scope, "a")
    end

    test "throw error when try get required value in plain key atom map" do
      scope = %{a: 1, b: 2, c: 3}
      assert {:error, :required} == Value.getr(scope, "d")

      assert_raise(RuntimeError, "Value of 'd' is required", fn ->
        Value.getr!(scope, "d")
      end)
    end

    test "get value in plain key atom map" do
      scope = %{a: 1, b: 2, c: 3}
      assert 1 == Value.get(scope, "a")
    end

    test "get value in plain key string map" do
      scope = %{"a" => 1, "b" => 2, "c" => 3}
      assert 1 == Value.get(scope, "a")
    end

    test "get value in deep key atom map" do
      scope = %{a: 1, b: 2, c: 3, d: %{a: 4}}
      assert 4 == Value.get(scope, "d.a")
    end

    test "get nil in deep key atom map when not exists field" do
      scope = %{a: 1, b: 2, c: 3, d: %{a: 4}}
      assert nil == Value.get(scope, "d.b")
    end

    test "get value in deep key string map" do
      scope = %{"a" => 1, "b" => 2, "c" => 3, "d" => %{"a" => 4}}
      assert 4 == Value.get(scope, "d.a")
    end

    test "get nil in deep key string map when not exists field" do
      scope = %{"a" => 1, "b" => 2, "c" => 3, "d" => %{"a" => 4}}
      assert nil == Value.get(scope, "d.b")
    end

    test "get value in deep key string or atom (mescled) map" do
      scope = %{"a" => 1, "b" => 2, "c" => 3, d: %{"a" => 4}}
      assert 4 == Value.get(scope, "d.a")
      scope = %{"a" => 1, "b" => 2, "c" => 3, d: %{"a" => 4, e: 7}}
      assert 7 == Value.get(scope, "d.e")
    end

    test "get values in deep key string map when field type atom or string" do
      scope = %{"a" => 1, "b" => [1, 2], "c" => 3, "d" => [%{"a" => 4}, %{a: 1, c: 1}]}
      assert [1, 2] == Value.get(scope, "b")
      assert [4, 1] == Value.get(scope, "d.a")
    end

    test "get multiple fields values" do
      scope = %{"a" => 1, "b" => [1, 2], "c" => 3, "d" => [%{"a" => 4}, %{a: 1, c: 1}]}
      assert %{"a" => 1, "b" => [1, 2]} == Value.getm(scope, "b,a")
      assert %{"b" => [1, 2], "d.a" => [4, 1]} == Value.getm(scope, "b,d.a")
    end
  end

  describe "Value.insert" do
    test "insert value in map" do
      scope = %{a: 1, b: 2, c: 3}
      assert Map.merge(scope, %{"ab" => 1}) == Value.insert(scope, "ab", 1)
    end

    test "replace value in deep key atom map" do
      scope = %{a: 1, b: 2, c: 3, d: %{a: 4}}
      assert %{a: 1, b: 2, c: 3, d: %{a: 3}} == Value.insert(scope, "d.a", 3)

      scope = %{a: 1, b: 2, c: 3, d: %{"a" => 4}}
      assert %{a: 1, b: 2, c: 3, d: %{"a" => 3}} == Value.insert(scope, "d.a", 3)
    end

    test "insert value on every field in field list on map " do
      scope = %{"d" => [%{"a" => 4}, %{a: 2}]}
      assert %{"d" => [%{"a" => 1}, %{a: 1}]} == Value.insert(scope, "d[@].a", 1)
    end

    test "insert value on index field in field list on map " do
      scope = %{"d" => [%{"a" => 4}, %{a: 2}]}
      assert %{"d" => [%{"a" => 1}, %{a: 2}]} == Value.insert(scope, "d[0].a", 1)
    end

    test "insert new field value on index field in field list on map " do
      scope = %{"d" => [%{"a" => 4}, %{a: 2}]}
      assert %{"d" => [%{"a" => 4, "c" => 1}, %{a: 2}]} == Value.insert(scope, "d[0].c", 1)
    end

    test "insert new field value on all list  " do
      scope = [%{"a" => 4}, %{a: 2}]
      assert [%{"a" => 4, "c" => 1}, %{"c" => 1, a: 2}] == Value.insert(scope, "_[*].c", 1)
    end
  end
end
