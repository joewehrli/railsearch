# railsearch 
## version 001 - brute force

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

### version 1  result 
Since query are larger than the data, the query in the outer loop is faster (qt4 qt6 faster than qt3 qt5 - oddness aside with other code changing results)

- queries are larger than data due since the have bool and don't care 


## version 2 - compress queries size and execution

## design problem 1
remove per query checking at each data visit

## idea 1 - and/or tree

reduce the number of operations slightly

Compile all active queries into a and/or true and do full traversal collecting data indexes for each query instance along the way

- QueryTerm can be interpreted as a rule P if p1, p2, ... pk where k need not to be a seq 1 to k, as missing prepositions are the don't cares

- Query is a list of QueryTerms, a composite query or rule set
P1 if p1, p2, ... pk
P2 if p1, p2, ... pk
these can be combined into a single and-or tree to evaluate against any record R(r1,r2,...,rk) or against record and other rules

- Ignore the composite query idea for now and going back to QueryTerm we can view active basket of querys on the rail as a set of QueryTerms, version 1 implements testing those

- If we combined all QueryTerms in a single AND-OR and traverse fully and evaluate a record, we can determine and assign the record to any succesful searches of the tree
```
P1 if r1, r2, r8
P2 if r2, r3, r4
P3 if r5, r7, r8

R r1, r2, ... rk
```
this does not save much
```
AG = P1 (Rx in result P1)
AG = P2 (Rx in result P1)
AG = P3 (Rx in result P1)

AG if-or-of P1,P2,P3
```
we don't know how much without some impl effort

## idea2 - build a trie of the rules and write custom evaluation

reduce the number of operations slightly

```
P1 if 1, 2, 8
P2 if 2, 3, 4
P3 if 5, 7, 8

R r1, r2, ... rk
```

```
R (null)-> 1,2,5
    1 -> 2
        2 -> 8
    2 -> 3
        3 -> 4
    5 -> 7
        7 -> 8
```

