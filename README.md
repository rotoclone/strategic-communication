# strategic-communication
A best-of-breed language with a holistic approach to moving the needle

## Register names
* customer experience
* revenue streams
* core competencies
* best practices
* stakeholder engagement
* key performance indicators
* return on investment
* assets

## Constants
* 0: HR
* 1: Engineering
* 2: Legal
* 3: PR
* 4: Finance
* 5: Marketing
* 6: R&D
* 7: Sales
* 8: TODO
* 9: TODO

## Operations (unless otherwise denoted, all operands must be register names)
* increment `x`:
  * innovate `x`
* multiply `x` by 10:
  * amplify `x`
* decrement `x`:
  * streamline `x`
* divide `x` by 10:
  * backburner `x`
* add `x` to `y` and store the result in `x`
  * synergize `x` with `y`
* subtract `y` from `x` and store the result in `x`
  * differentiate `x` from `y`
* define a label called `x`
  * moving forward, `x**`
* jump to label `x`
  * circle back to `x**`
* jump to label `x` if the value in `y` is zero
  * pivot `y` to `x**`
* jump to label `x` if the value in `y` is negative
  * restructure `y` to `x**`
* set `x` to `y`
  * align `x` with `y*`
* print `x` to stdout (in UTF-8 encoding)
  * deliver `x`
* set `x` to zero
  * overhaul `x`
* set `x` to a random number between 0 and 9 inclusive
  * paradigm shift `x`
* read a single UTF-8 character from stdin and store it in `x`
  * crowdsource `x`
* exit
  * take it offline

\* can be either a register name or a constant expression

\** can be any string containing no reserved words

## Constant expressions
A constant expression is a sequence of one or more constants separated by `,` or `and`. The value of the expression is the result of concatenating the values of the constants. For example, `Engineering` has a value of `1` and `Marketing` has a value of `5`, so the expression `Engineering and Marketing` has a value of `15`.

More examples:
* `Engineering, Marketing, and HR` = `150`
* `Engineering` = `1`
* `HR and Engineering` = `1` (leading zeros are ignored)
* `Marketing, Marketing, Marketing` = `555`

## Example program
This program prints the values 0 to 10 to stdout:
```
TODO
```
