# xgl

MySQL General Query Log parser.

[![Build Status](https://github.com/winebarrel/xgl/workflows/test/badge.svg?branch=main)](https://github.com/winebarrel/xgl/actions)

## Usage

```
$ cat general.log
2020-05-27T05:03:27.500301Z   11 Query	SET @@sql_log_bin=off
2020-05-27T05:03:27.543379Z   11 Query	select @@session.tx_read_only
2020-05-27T05:03:27.683485Z   11 Query	COMMIT
...

$ xgl general.log # or `cat general.log | xgl`
{"Time":"2020-05-27T05:03:27.500301Z","Id":"11","Command":"Query","Argument":"SET @@sql_log_bin=off"}
{"Time":"2020-05-27T05:03:27.543379Z","Id":"11","Command":"Query","Argument":"select @@session.tx_read_only"}
{"Time":"2020-05-27T05:03:27.683485Z","Id":"11","Command":"Query","Argument":"COMMIT"}
...
```

## Related Links

* https://github.com/winebarrel/genlog
