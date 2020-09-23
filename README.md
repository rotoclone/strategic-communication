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
More examples can be found in the [examples](examples) directory.

## Register names
There are 8 available registers. Each one starts with a value of 0, and can hold any 32-bit signed integer. They are named as follows:
* customer experience
* revenue streams
* core competencies
* best practices
* stakeholder engagement
* key performance indicators
* return on investment
* assets

## Constants
There are 10 constants used to represent literal numbers (more information on using these can be found in the [Constant expressions](#constant-expressions) section below):
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
A Strategic Communication program consists of a series of operations separated by newlines.

Unless otherwise denoted, all operands must be register names.
|Description|Formats|Notes|
|-----------|-------|-----|
|increment the value in `x`|<ul><li>`innovate x`</li><br><li>`value-add x`</li></ul>||
|decrement the value in `x`|<ul><li>`streamline x`</li><br><li>`optimize x`</li></ul>||
|multiply the value in `x` by -1|<ul><li>`revamp x`</li><br><li>`overhaul x`</li></ul>||
|multiply the value in `x` by 2|<ul><li>`amplify x`</li><br><li>`incentivize x`</li></ul>||
|divide the value in `x` by 2|<ul><li>`backburner x`</li></ul>|any remainder is discarded|
|set the value in `x` to a random number between 0 and 9 inclusive|<ul><li>`paradigm shift x`</li></ul>||
|set the value in `x` to the value in `y`|<ul><li>`align x with y`</li></ul>|`y` can be a [register name](#register-names) or a [constant expression](#constant-expressions)|
|add the value in `x` to the value in `y` and store the result in `x`|<ul><li>`synergize x and y`</li><br><li>`integrate x and y`</li></ul>||
|subtract the value in `y` from the value in `x` and store the result in `x`|<ul><li>`differentiate x and y`</li></ul>||
|read a single byte from stdin and store it in `x`|<ul><li>`crowdsource x`</li></ul>|if EOF is encountered, the value in `x` is set to -1|
|print the value in `x` to stdout|<ul><li>`deliver x`</li><br><li>`produce x`</li></ul>|UTF-8 encoding will be used|
|define a label called `x`|<ul><li>`moving forward, x`</li><br><li>`going forward, x`</li></ul>|`x` can be any string containing no [register names](#register-names) or [constants](#constants)|
|jump to label `x`|<ul><li>`circle back to x`</li><li>`revisit x`</li></ul>|`x` must be a defined label|
|jump to label `x` if the value in `y` is zero|<ul><li>`pivot y to x`</li></ul>|`x` must be a defined label|
|jump to label `x` if the value in `y` is negative|<ul><li>`restructure y to x`</li></ul>|`x` must be a defined label|

## Constant expressions
A constant expression is a sequence of one or more constants separated by `,` or `and`. The value of the expression is the result of concatenating the values of the constants. For example, `Engineering` has a value of `1` and `Marketing` has a value of `5`, so the expression `Engineering and Marketing` has a value of `15`.

More examples:
* `Engineering, Marketing, and HR` = `150`
* `Engineering` = `1`
* `HR and Engineering` = `1` (leading zeros are ignored)
* `Marketing, Marketing, Marketing` = `555`

## Comments
The syntax of Strategic Communication meets or exceeds the highest standards of corporate discourse, therefore comments are unnecessary and not supported.

## Running a program
To run a Strategic Communication program, download the interpreter for your system from the [latest release](https://github.com/rotoclone/strategic-communication/releases) and provide the path to the source code when running the command.

### Windows
```
strategic-communication.exe examples\hello_world.business
```

### macOS/Linux
```
./strategic-communication examples/hello_world.business
```

## Building the interpreter
1. Clone this repo
2. [Install Rust](https://www.rust-lang.org/tools/install)
3. `cargo build`
