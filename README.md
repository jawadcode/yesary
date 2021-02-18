# yesary

## What is this?
This is a program that encodes a file using the "yesary" encoding where each byte is broken down into 3 octal digits which different capitalisations of "yes" to represent each digit:

    0 => "yes"
    1 => "yeS"
    2 => "yEs"
    3 => "yES"
    4 => "Yes"
    5 => "YeS"
    6 => "YEs"
    7 => "YES"

##### Note: The digit can also be converted to binary using a capitalised letter as 1 and a lowercase letter as 0, e.g. "YEs" => 110

## Features:
- [x] Encode file
- [ ] Decode file
- [ ] Take in flags so it can be used as CLI toole