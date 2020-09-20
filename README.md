# strategic-communication
A best-of-breed language with a holistic approach to moving the needle.

## Example program
This program prints the numbers 0 to 9 to stdout separated by newlines:
```
align stakeholder engagement with Engineering and HR
align revenue streams with stakeholder engagement
revamp revenue streams
align customer experience with Finance and Manufacturing
moving forward, think outside the box
deliver customer experience
deliver stakeholder engagement
innovate customer experience
innovate revenue streams
restructure revenue streams to think outside the box
```
Translated to pseudocode:
```
stakeholder_engagement = 10 // '\n'
revenue_streams = stakeholder_engagement * -1
customer_experience = 48 // '0'
do {
  print(customer_experience)
  print(stakeholder_engagement)
  customer_experience++
  revenue_streams++
} while revenue_streams < 0
```

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
* 8: Manufacturing
* 9: Executive Management

## Operations
Unless otherwise denoted, all operands must be register names.
* increment `x`
  * innovate `x`
* decrement `x`
  * streamline `x`
* multiply `x` by -1
  * revamp `x`
* multiply `x` by 2
  * amplify `x`
* divide `x` by 2 (throwing away remainder)
  * backburner `x`
* set `x` to zero
  * overhaul `x`
* set `x` to a random number between 0 and 9 inclusive
  * paradigm shift `x`
* set `x` to `y`
  * align `x` with `y*`
* add `x` to `y` and store the result in `x` (TODO)
  * synergize `x` with `y`
* subtract `y` from `x` and store the result in `x` (TODO)
  * differentiate `x` from `y`
* read a single UTF-8 character from stdin and store it in `x` (TODO)
  * crowdsource `x`
* print `x` to stdout (in UTF-8 encoding)
  * deliver `x`
* define a label called `x`
  * moving forward, `x**`
  * going forward, `x**`
* jump to label `x` (TODO)
  * circle back to `x**`
* jump to label `x` if the value in `y` is zero (TODO)
  * pivot `y` to `x**`
* jump to label `x` if the value in `y` is negative
  * restructure `y` to `x**`
* exit (TODO)
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

## Comments
The syntax of Strategic Communication meets or exceeds the highest standards of corporate discourse, therefore comments are unnecessary and not supported.
