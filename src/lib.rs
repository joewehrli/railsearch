#![allow(dead_code)]

#[cfg(foo)]
const MAX_RECORDS: usize = 800_000_000;

const MAX_RECORDS: usize = 50_000;
const MAX_QUERIES: usize = 10_000;

const MAX_FLAGS: usize = 10;

// a sequence of signed integers is the key
// seq 1,2,5 represents the rule/query body Pn if r1,r2,r5 
//  where r1,r2,r5 are boolean condition tests of T or F applied 
//      to the datarow d1,d2,..,d5,,..dmax,  items not referenced by rule like d3 d4 ... are don't care conditions.. and are not in the sequence
//      n indicates the the nth predicate rN is true, -n indicates the predicate rN is false
//      -n implements negation of nth boolean variable in the data and equivalently expression negation to the term not(rN)
// valid rules in sequence are <-128, 127>
#[derive(PartialEq, Eq, Clone, Copy)]
struct RuleMashTrieKey{
    seq :[i8; MAX_FLAGS]
}

// a trie like structure that mashes together business rules for mashed evaluation
struct RuleMashTrie {
    null_structure : bool, 
    data_structure : bool, 
    rule_xref : Vec<usize>, 
    k : RuleMashTrieKey,
    //MAX_FLAGS = alphabet of size 10; 10 means rules <1,10> with index <0,9>
    children : Vec<RuleMashTrie>
}

impl RuleMashTrie {
    fn new() -> Self {

        let s = [0; MAX_FLAGS];
        let x = RuleMashTrieKey {seq :s };

        RuleMashTrie { 
            null_structure : true,
            data_structure : true,
            rule_xref : Vec::new(),
            k : x,
            children: Vec::new()
          }
    }

/*
trie insert( key, t, depth )
	typekey key;
	trie t;
	int depth;

	{
	int j;
	trie t1;
	if ( t==NULL )	return( NewDataNode(key) );
	if ( IsData(t) )
		if (t->k == key)
			Error /*** Key already in table ***/;
		else {	t1 = NewIntNode();
			t1->p[ charac(depth,t->k) ] = t;
			t = insert( key, t1, depth );
			}
	else  { j = charac(depth,key);
		t->p[j] = insert( key, t->p[j], depth+1 );
		}
	return( t );
	}
*/
    //returning Tree by value is not the desire here

    fn insert(key: &RuleMashTrieKey, trie: &mut RuleMashTrie, depth: usize, rule_id : usize) -> RuleMashTrie {
        type T = RuleMashTrie;

        if T::is_null(trie)	{
            // make_new_data_node
            let mut trie_root : T = T::new();
            trie_root.null_structure=false;
            trie_root.k = *key;
            trie_root.data_structure = true;
            trie_root.rule_xref.push(rule_id);
            return trie_root;
        }

        if T::is_data(trie) {
            if trie.k == *key {
                // Key already in table push new ruleid
                trie.rule_xref.push(rule_id);
                return T::new();
            }
            else{
                // make_new_interior_node
                return T::new();
            }
        }
        else {
            // walk more
            let j : usize = T::charac(depth, key) as usize;
            let nextsubtree = &mut trie.children[j];
            let subtree = T::insert(key, nextsubtree, depth, rule_id);
            trie.children[j] = subtree;
            return trie; // how to return this?
        }
    }
/*
search( key, t )
	typekey key;
	trie t;

	{
	int depth;
	for( depth=1; t!=NULL && !IsData(t); depth++ )
		t = t->p[ charac(depth,key) ];
	if( t != NULL && key == t->k )
		found( t );
	else	notfound( key );
	}
*/
    fn search (key: &RuleMashTrieKey, trie: &RuleMashTrie){
        let mut depth: usize = 1;
        type T = RuleMashTrie;
        let mut trie_walk : &T = trie;
        while !T::is_null(trie) && !T::is_data(trie) {
            let next_index : usize= T::charac(depth, key) as usize;
            trie_walk = &trie_walk.children[next_index];
            depth = depth + 1;
        }
        if !T::is_null(trie) && key == &trie.k {
                T::found( trie );
        }
        else {
            T::notfound(key);
        }
    }

    fn charac(depth: usize, key: &RuleMashTrieKey) -> i8{
        assert!(depth>0);
        let idx = depth-1;
        let val = key.seq[idx];
        return val;
    }

    fn is_null(trie: &RuleMashTrie) -> bool {
        return trie.null_structure;
    }

    fn is_data(trie: &RuleMashTrie) -> bool {
        return trie.data_structure;
    }

    fn found (_trie: &RuleMashTrie) {
        println!("Search key FOUND");
    }
    fn notfound (_key: &RuleMashTrieKey) {
        println!("Search key NOT FOUND");
    }

    /*
    fn make_node() -> RuleMashTrie {
        let child_vec: Vec<RuleMashTrie> = vec![query_terms_init; MAX_QUERIES];
        let mut node = RuleMashTrie {
            predicate_index : 0,
            children: Vec::new()
        }
    }
*/

}

pub fn trie_test() {

    type T = RuleMashTrie;

    let t = T::new();

    let mut s = [0; MAX_FLAGS];
    s[0]=1;
    let k = RuleMashTrieKey {seq :s };

    T::search(&k, &t);
}


// simple FIXED bool record layout
// later internal optimization -  bitwise ops - 256x more storage

#[derive(Clone, Copy)]
struct Fixed {
    flags : [bool; MAX_FLAGS]  
}
impl Fixed {

    fn new(flags : [bool; MAX_FLAGS]) -> Self {
        Fixed { flags  }
    }

    fn test2(self: &Fixed, query_term : &QueryTerm) -> bool {

        let mut test_result: bool = true; // assume a match
        for x in 0..MAX_FLAGS {
            if query_term.active_terms[x] != false { // skip any don't care positions/flags
                if query_term.match_terms[x] != self.flags[x] // term not match
                {
                    test_result = false; // no match
                    break; // break if no match
                }
            }
        }
        return test_result;
    }


}

// simple query model for bools QueryTerms
// later internal optimization -  bitwise ops - 256x more storage

#[derive(Clone, Copy)]
struct QueryTerm {
    match_terms : [bool; MAX_FLAGS], // ith is true means that ith data should be true, unless ith active_terms is false
    active_terms : [bool; MAX_FLAGS]
}
impl QueryTerm {

    fn test1(self: QueryTerm, flags : [bool; MAX_FLAGS]) -> bool {

        let mut test_result: bool = true; // assume a match
        for x in 0..MAX_FLAGS {
            if self.active_terms[x] != false { // skip any don't care positions/flags
                if self.match_terms[x] != flags[x] // term not match
                {
                    test_result = false; // no match
                    break; // break if no match
                }
            }
        }
        return test_result;
    }

    fn test2(self: &QueryTerm, flags : &[bool; MAX_FLAGS]) -> bool {

        let mut test_result: bool = true; // assume a match
        for x in 0..MAX_FLAGS {
            if self.active_terms[x] != false { // skip any don't care positions/flags
                if self.match_terms[x] != flags[x] // term not match
                {
                    test_result = false; // no match
                    break; // break if no match
                }
            }
        }
        return test_result;
    }

}

pub fn queryterm1(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true; 
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for r in 0..MAX_RECORDS {
        let data = fixed_vec[r].flags;
        for q in 0..MAX_QUERIES {
            let query = queries_vec[q]; 
            let test = query.test1(data);
            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}


pub fn queryterm2(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true; 
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for q in 0..MAX_QUERIES {
        let query = queries_vec[q]; 
        for r in 0..MAX_RECORDS {
            let data = fixed_vec[r].flags;        
            let test = query.test1(data);
            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}

pub fn queryterm3(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true; 
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for r in 0..MAX_RECORDS {
        let data = fixed_vec[r].flags;
        for q in 0..MAX_QUERIES {
            let query = queries_vec[q]; 
            let test = query.test2(&data);
            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}


pub fn queryterm4(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true; 
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for q in 0..MAX_QUERIES {
        let query = queries_vec[q]; 
        for r in 0..MAX_RECORDS {
            let data = fixed_vec[r].flags;        
            let test = query.test2(&data);
            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}

pub fn queryterm5(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true; 
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for r in 0..MAX_RECORDS {
        let data = fixed_vec[r];//diff than 3 4
        for q in 0..MAX_QUERIES {

            let query = queries_vec[q];//diff than 3 4
            let test = data.test2(&query);//diff than 3 4

            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}


pub fn queryterm6(){
    let fixed_vec: Vec<Fixed> = build_data();

    let mut query_terms_init = QueryTerm { 
        match_terms: [false; MAX_FLAGS], 
        active_terms: [false; MAX_FLAGS] 
    };

    query_terms_init.match_terms[0]=true;
    query_terms_init.active_terms[0]=true;

    let queries_vec: Vec<QueryTerm> = vec![query_terms_init; MAX_QUERIES];
    let mut true_count = 0;
    let mut false_count = 0;

    for q in 0..MAX_QUERIES {
        let query = queries_vec[q]; 
        for r in 0..MAX_RECORDS {

            let data = fixed_vec[r]; //diff than 3 4       
            let test = data.test2(&query); //diff than 3 4

            if test { //passed query term test so include result
                true_count = true_count + 1;
            }
            else {
                false_count = false_count + 1;
            }
        }
    }

    assert_eq!(true_count, (MAX_RECORDS/2) * MAX_QUERIES);
    assert_eq!(false_count, (MAX_RECORDS/2) * MAX_QUERIES);
}



fn check_data(fixed_vec : &Vec<Fixed>) {
    let mut last = 0;
    let mut true_count = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }
    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);

    return;
}

// shared data builder for query tests

fn build_data() -> Vec<Fixed> {

    let fixed_init = Fixed { flags: [false; MAX_FLAGS] };
    let mut fixed_vec: Vec<Fixed> = vec![fixed_init; MAX_RECORDS];

    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    check_data(&fixed_vec);

    return fixed_vec;
}



// data layout models

// model 3 4 5 about same and edge - see performance.md

pub fn model1(){
    let t_flags : [bool; MAX_FLAGS] = [false; MAX_FLAGS];

    // fixed_vec has a capacity of MAX_RECORDS - it can reallocate more if push more
    let mut fixed_vec = Vec::with_capacity(MAX_RECORDS);

    for _ in 0..MAX_RECORDS {
        fixed_vec.push(Fixed::new(t_flags));
    }

    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    let mut true_count = 0;
    last = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }

    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);
}

pub fn model2(){
    let t_flags : [bool; MAX_FLAGS] = [false; MAX_FLAGS];

    // fixed_vec has a capacity of MAX_RECORDS - it can reallocate more if push more
    let mut fixed_vec: Vec<Fixed> = Vec::with_capacity(MAX_RECORDS);

    let fixed_init: Fixed  = Fixed::new(t_flags);
    for _ in 0..MAX_RECORDS {
        fixed_vec.push(fixed_init);
    }

    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    let mut true_count = 0;
    last = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }

    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);
}

pub fn model3(){
    let t_flags : [bool; MAX_FLAGS] = [false; MAX_FLAGS];
    let fixed_init: Fixed  = Fixed::new(t_flags);
    // fixed_vec has a capacity of MAX_RECORDS - it can reallocate more if push more
    let mut fixed_vec: Vec<Fixed> = vec![fixed_init; MAX_RECORDS];


    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    let mut true_count = 0;
    last = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }

    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);
}

pub fn model4(){
    let t_flags : [bool; MAX_FLAGS] = [false; MAX_FLAGS];
    let fixed_init = Fixed { flags: t_flags };
    // fixed_vec has a capacity of MAX_RECORDS - it can reallocate more if push more
    let mut fixed_vec: Vec<Fixed> = vec![fixed_init; MAX_RECORDS];


    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    let mut true_count = 0;
    last = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }

    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);
}

pub fn model5(){
    let fixed_init = Fixed { flags: [false; MAX_FLAGS] };
    let mut fixed_vec: Vec<Fixed> = vec![fixed_init; MAX_RECORDS];


    let mut last = 0;
    for x in 0..MAX_RECORDS {
        if x >= MAX_RECORDS/2 {
            break;
        }
        last = x;
        fixed_vec[x].flags[0]=true;
    }
    assert_eq!(last, MAX_RECORDS/2 - 1);

    let mut true_count = 0;
    last = 0;
    for x in 0..MAX_RECORDS {

        if fixed_vec[x].flags[0] == true {

            true_count +=1;
        }
        last = x;
    }

    assert_eq!(true_count, MAX_RECORDS/2);
    assert_eq!(last, MAX_RECORDS - 1);
}
