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

