# gq - Get Quote

`gq` is a small standalone tool to fetch the last closing price of a
stock. It uses Yahoo Finance's API to get quote prices and does not
require any authentication.

```sh
$ gq TSLA GOOG
GOOG 1421.59
TSLA 833.79
```

Quotes that are not found will be ignored.

I made this script because I wanted something simple to pull quotes
from the command line.
