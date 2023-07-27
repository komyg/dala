# Dala Formula Language

The Dala Formula Language is a light weight language, that is heavily inspired by Excel and Google Sheets formulas, for example: `SUM(1, 2, 3, 4, 5)` will yield `15`.

It is currently a work in progress.

## Primitives

These primitive values are used as arguments and as return values from functions.

| Primitive | Description                                      | Syntax               |
| --------- | ------------------------------------------------ | -------------------- |
| `Str`     | A string of characters enclosed by double quotes | `"Hello World"`      |
| `Num`     | A number                                         | `1`, `2.5`, `3.1415` |
| `Boolean` | A boolean value                                  | `TRUE`, `FALSE`      |

### `EQ(a, b) -> Boolean`

#### Description

Returns `TRUE` if the arguments are equal, otherwise returns `FALSE`.

#### Arguments

| Name | Description                    | Type                        |
| ---- | ------------------------------ | --------------------------- |
| a    | The first argument to compare  | `Str` or `Num` or `Boolean` |
| b    | The second argument to compare | `Str` or `Num` or `Boolean` |

#### Return Value

| Type      | Description           |
| --------- | --------------------- |
| `Boolean` | The comparison result |

#### Example

```dala
EQ(1,1) -> TRUE

```

### `NEQ(a, b) -> Boolean`

#### Description

Returns `TRUE` if the arguments are different, otherwise returns `FALSE`.

#### Arguments

| Name | Description                    | Type                        |
| ---- | ------------------------------ | --------------------------- |
| a    | The first argument to compare  | `Str` or `Num` or `Boolean` |
| b    | The second argument to compare | `Str` or `Num` or `Boolean` |

#### Return Value

| Type      | Description           |
| --------- | --------------------- |
| `Boolean` | The comparison result |

#### Example

```dala
EQ(1,1) -> FALSE

```

### `CONCAT(a, b, c, ...) -> Str`

#### Description

Concatenates all the arguments together into a single string.

#### Arguments

This function takes one or more arguments.

Non `Str` arguments will be implictly converted to `Str`, before being concatenated.

| Name | Description                            | Type                        |
| ---- | -------------------------------------- | --------------------------- |
| a    | The first string to concat             | `Str` or `Num` or `Boolean` |
| b    | The second string to concat (optional) | `Str` or `Num` or `Boolean` |
| ...  | The rest of the strings (optional)     | `Str` or `Num` or `Boolean` |

#### Return Value

| Type  | Description                                   |
| ----- | --------------------------------------------- |
| `Str` | All arguments concatenated as a single string |

#### Example

```dala
CONCAT("Hello", " ", "World") -> "Hello World"
```

### `DIVIDE(a, b) -> Num`

#### Description

Divide the first argument by the second one.

#### Arguments

| Name | Description  | Type  |
| ---- | ------------ | ----- |
| a    | The dividend | `Num` |
| b    | The divisor  | `Num` |

#### Return Value

| Type  | Description         |
| ----- | ------------------- |
| `Num` | The division result |

#### Example

```dala
DIVIDE(10, 2) -> 5
```

### `IF(condition, if_true, if_false) -> Literal`

#### Description

If the condition is true, evaluates the `if_true`, otherwise evaluates the `if_false`.

#### Arguments

This function takes one or more arguments.

Non `Str` arguments will be implictly converted to `Str`, before being concatenated.

| Name      | Description                                | Type                        |
| --------- | ------------------------------------------ | --------------------------- |
| condition | Boolean or function that returns a boolean | `Boolean`                   |
| if_true   | Evaluated if `condition` is `TRUE`         | `Str` or `Num` or `Boolean` |
| if_false  | Evaluated if `condition` is `FALSE`        | `Str` or `Num` or `Boolean` |

#### Return Value

| Type                        | Description                                                             |
| --------------------------- | ----------------------------------------------------------------------- |
| `Str` or `Num` or `Boolean` | The result of evaluating the contents of either `if_true` or `if_false` |

#### Example

```dala
`IF(TRUE, "Hello", "World") -> "World"`
```

### `MULTIPLY(a, b) -> Num`

#### Description

Multiplies the arguments.

#### Arguments

| Name | Description         | Type  |
| ---- | ------------------- | ----- |
| a    | The first argument  | `Num` |
| b    | The second argument | `Num` |

#### Return Value

| Type  | Description               |
| ----- | ------------------------- |
| `Num` | The multiplication result |

#### Example

```dala
MULTIPLY(5, 2) -> 10
```

### `SUBTRACT(a, b) -> Num`

#### Description

Subtract the second argument from the first one.

#### Arguments

This function takes one or more arguments.

| Name | Description         | Type  |
| ---- | ------------------- | ----- |
| a    | The first argument  | `Num` |
| b    | The second argument | `Num` |

#### Return Value

| Type  | Description            |
| ----- | ---------------------- |
| `Num` | The subtraction result |

#### Example

```dala
SUBTRACT(1, 2) -> -1
```

### `SUM(a, b, c, ...) -> Num`

#### Description

Sums all the arguments together.

#### Arguments

This function takes one or more arguments.

| Name | Description                             | Type  |
| ---- | --------------------------------------- | ----- |
| a    | The first number to sum                 | `Num` |
| b    | The second number to sum (optional)     | `Num` |
| ...  | The rest of the numbers to sum (option) | `Num` |

#### Return Value

| Type  | Description             |
| ----- | ----------------------- |
| `Num` | All arguments summed up |

#### Example

```dala
SUM(1, 2, 3, 4, 5) -> 15
```

### `UPPER(a) -> Str`

#### Description

Converts the string to upper case.

#### Arguments

Non `Str` arguments will be implictly converted to `Str`, before being concatenated.

| Name | Description         | Type                        |
| ---- | ------------------- | --------------------------- |
| a    | The string to upper | `Str` or `Num` or `Boolean` |

#### Return Value

| Type  | Description        |
| ----- | ------------------ |
| `Str` | The upper case str |

#### Example

```dala
UPPER("Hello World") -> "HELLO WORLD"
```
