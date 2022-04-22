# Value

**Get and Insert value on map without complexity.**


## Get Value

The Value.get function works getting the data from a map or deep map

Example 1

```elixir
 iex(1)> map = %{a: 1, b: 2, c: 3}
 iex(2)> Value.get(map, :a)
 1
```

Example 2

```elixir
 iex(1)> map = %{a: 1, b: 2, c: 3, d: %{"e" => 1, c: %{d: 2}}}
 iex(2)> Value.get(map, "d.c.d")
 2
```

Example 3

```elixir
 iex(1)> map = %{a: 1, b: 2, c: [%{a: 1, b: 2}, %{a: 5, c: 6}]}
 iex(2)> Value.get(map, "c.a")
 [1, 5]
```

Example 4

```elixir
 iex(1)> map = %{a: 1, b: 2, c: [%{a: 1, b: 2}, %{a: 5, c: 6}]}
 iex(2)> Value.get(map, "c.a[0]")
 1
```
## Insert Value

The Value.insert function works inserting the data from a map or deep map

Example 1

```elixir
 iex(1)> map = %{a: 1, b: 2, c: 3}
 iex(2)> Value.insert(map, "a", 2)
 %{a: 2, b: 2, c: 3}
```

Example 2

```elixir
 iex(1)> map = %{a: 1, b: 2, c: 3}
 iex(2)> Value.insert(map, "d.c.d", 4)
 %{:a => 1, :b => 2, :c => 3, "d" => %{"c" => %{"d" => 4}}}
```

Example 3

```elixir
 iex(1)> map = %{a: 1, b: 2, c: [%{a: 1, b: 2}, %{a: 5, c: 6}]}
 iex(2)> Value.insert(map, "c[@].c", 2)
 %{a: 1, b: 2, c: [%{:a => 1, :b => 2, "c" => 2}, %{a: 5, c: 2}]}
```

Example 4

```elixir
 iex(1)> map = %{a: 1, b: 2, c: [%{a: 1, b: 2}, %{a: 5, c: 6}]}
 iex(2)> Value.insert(map, "c[0].a",2)
 %{a: 1, b: 2, c: [%{a: 2, b: 2}, %{a: 5, c: 6}]}
```
