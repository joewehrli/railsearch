railsearch - version 001

`record` is a bit array of fixed

`query` is ?

`and` / `or` of a `query terms` 

`query term` is a pattern of qt [0,MAX] where qt[i] is in { true, false, nocare }


railsearch takes a  `basket of actives queries`  and scans for records that satisfy one of those

version 1 will not use ast true - just a large array of query patterns to spin through and add the record index to

Concept to show:
rail-search beats linear search above some `threshold of basket size` of concurrent active queries
the price we pay with respect to:
best/avg/worst/actual latency (time to get answer)
best/avg/worst/actual operations (algorithm time/steps)
best/avg/worst/actual operations (space)
it and cloud deployment costs

version 1  result - since query are larger than the data, the query in the outer loop is faster (qt4 qt6 faster than qt3 qt5 - oddness aside with other code changing results)


version 2 - compress queries size and execution

idea - compile all active queries into a and/or true and do full traversal collecting data indexes for each query instance along the way

design problem 1 - remove per query checking at each, data visit
