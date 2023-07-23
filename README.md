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

## Functions

Each function can take primitive or other functions as arguments.

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
