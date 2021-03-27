# railsearch

2021-03-01
trying idea 2
started on specific trie structure - WIP
left status, it compiles

2021-03-02
WIP - no compile

2021-03-02
adds and to compile
completed search
insert in progress

2021-03-04
made insert compile
added print
insert has infinite loop
checked in with this

tree insert not correct - second insert has tree ref after as null_structure

2021-03-06
added logging
macro

2021-03-07
fixed up tree
tested ops in detail looks good
all but the wasted 0 child array :)
and no support for negation -x
checked in with the limitations

2021-03-13
adjusted libraries org (added some oldlib oldmain and setforth new cleaner libs)
checked in
adjusted to use child array position 0
checked in
move the trie test method to a test method
checkin
now design a way to handle negation in the rule 
here we use -negative integer to represent the negation of the ith integer variable

we were doing [4,3,2,1] maps to child node @ index - 1

MAX flags even
child -MAX/2, -MAX+1, -MAX+2,...0?, ... MAX/2-1, MAX/2

MAX = 6
-6/2-0(-3), -6/2+1(-2), -6/2+2(-1),  6/2-2(1), 6/2-1(2), 6/2-0(3)

F alpha -> array_pos
a < 0
-3 0  a + max/2 + 0 = 0     or -3 + 3 = 0
-2 1  a + max/2 + 1 = 1     or -2 + 3 = 1
-1 2  a + max/2 + 2 = 2     or -1 + 3 = 2

a > 0
1 3     a + max/2 - 1 = 3   or 1 + 3 - 1 = 3
2 4     a + max/2 - 1 = 4   or 2 + 3 - 1 = 4
3 5     a + max/2 - 1 = 5   or 3 + 3 - 1 = 5

2021-03-14
adjusted scheme to include negative terms and adjusted units tests and passed
checkin
added some negative terms, tested, and added into unit test

IF[1, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[1]
IF[1, 3, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[13]
IF[2, -3, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[230000000]
IF[2, 1, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[21]
IF[2, 2, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[22]
IF[2, 3, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[23]
IF[3, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[3, 3000]
IF[3, -1, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[310000000, 3100000002]
IF[3, 1, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[31, 3100]

checkin

sketch eval method; does it needs to have alphabet twice the bit length of the data bit length? no, no need to allow contradictions

thus adjusted constant MAX_FLAGS to MAX_SYM in rule_tree

as a result had to boost logging sizes

started to implement a eval

2021-03-23
reimplemeneted eval correctly - but crashed

2021-03-25
bit string seems to need to be half the alphabet - was wrong earlier - back to this

blow up npos -17 or 17

rule expression length
and
data bit size
BOTH
are really 1/2 the alphabet size, since contradiction rules don't need to expressed)
adjusted to get eval infinite loop

2021-03-27
debug infinite loop eval
typo caused loop trie vs trie1 in eval body

first run -> correct

rule dump
IF[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[1]
IF[1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[13]
IF[2, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[230000000]
IF[2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[21]
IF[2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[22]
IF[2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[23]
IF[3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[3, 3000]
IF[3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[310000000, 3100000002]
IF[3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[31, 3100]

match bits=[t,t,f,f,f,...
rule-equiv=[1, 2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
IF[2, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[230000000]
IF[2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[22]
IF[2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[21]
IF[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] THEN Q[1]

checkin

