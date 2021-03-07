#![allow(dead_code)]

#[path = "util.rs"]
mod util;

use crate::util::SemanticProducer;

#[cfg(foo)]
const MAX_RECORDS: usize = 800_000_000;

const MAX_RECORDS: usize = 50_000;
const MAX_QUERIES: usize = 10_000;

const MAX_FLAGS: usize = 10;


macro_rules! log56 {
    ( $( $x:expr, $y:expr ), *) => {
        {
            $(
                crate::util::SemanticProducer::pad56(&mut $y);
                crate::util::SemanticProducer::produce56($x, $y.as_bytes());
            )*
        }
    };    
}

// a sequence of signed integers is the key
// seq 1,2,5 represents the rule/query body Pn if r1,r2,r5 
//  where r1,r2,r5 are boolean condition tests of T or F applied 
//      to the datarow d1,d2,..,d5,,..dmax,  items not referenced by rule like d3 d4 ... are don't care conditions.. and are not in the sequence
//      n indicates the the nth predicate rN is true, -n indicates the predicate rN is false
//      -n implements negation of nth boolean variable in the data and equivalently expression negation to the term not(rN)
// valid rules in sequence are <-128, 127>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RuleMashTrieKey{
    seq :[i8; MAX_FLAGS]
}

// a trie like structure that mashes together business rules for mashed evaluation
#[derive(Clone)]
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
            data_structure : false,
            rule_xref : Vec::new(),
            k : x,
            children: Vec::new()
          }
    }


    
    fn set_data_node(key: &RuleMashTrieKey, trie: &mut RuleMashTrie, rule_id : usize){
        trie.null_structure=false;
        trie.data_structure = true;
        trie.k = *key;
        trie.rule_xref.push(rule_id);
    }

    fn set_interior_node(trie: &mut RuleMashTrie){
        trie.null_structure=false;
        trie.data_structure = false;
        //trie.k = *key;
        let new_mash = RuleMashTrie::new();
        trie.children = vec![new_mash; MAX_FLAGS];
    }

    fn copy_node(trie: &mut RuleMashTrie, triecopy: &mut RuleMashTrie) {
        triecopy.null_structure=trie.null_structure;
        triecopy.data_structure = trie.data_structure;
        triecopy.k = trie.k;
        triecopy.children = trie.children.clone(); 
        triecopy.rule_xref = trie.rule_xref.clone(); 
    }

    fn print_node (trie: &RuleMashTrie){
        println!("BGN print_node()");
        let stuff_str = format!("{:?}", trie.k);
        println!("key: {}", stuff_str);
        let stuff_str = format!("{:?}", trie.rule_xref);
        println!("Rule-ids= {}", stuff_str);
        println!("null_structure= {}", trie.null_structure);
        println!("data_structure= {}", trie.data_structure);
        println!("END print_node()");
        println!("");
    }

/*
void print_trie(TrieNode* root) {
    // Prints the nodes of the trie
    if (!root)
        return;
    TrieNode* temp = root;
    printf("%c -> ", temp->data);
    for (int i=0; i<N; i++) {
        print_trie(temp->children[i]); 
    }
}
*/

fn print (trie: &RuleMashTrie, depth: usize){
    println!("BGN print ({})", depth);
    type T = RuleMashTrie;

    if T::is_null(trie) {
        println!("Null - END print ({})", depth);
        println!("");
        return;
    }

    if T::is_data(trie) {
        println!("Data:");
        println!("Depth= {}", depth);
        let stuff_str = format!("{:?}", trie.k);
        println!("Key= {}", stuff_str);
        let stuff_str = format!("{:?}", trie.rule_xref);
        println!("rule-ids= {}", stuff_str);
    }
    else {
        let mut i = 0;
        while i < MAX_FLAGS {
            T::print( &trie.children[i], depth + 1 );
            i = i + 1;
        }
    }
    println!("END print ({})", depth);
    println!("");
    return;
}

/*
// If not present, inserts key into trie 
// If the key is prefix of trie node, just 
// marks leaf node 
void insert(struct TrieNode *root, string key) 
{ 
    struct TrieNode *pCrawl = root; 
  
    for (int i = 0; i < key.length(); i++) 
    { 
        int index = key[i] - 'a'; 
        if (!pCrawl->children[index]) 
            pCrawl->children[index] = getNode(); 
  
        pCrawl = pCrawl->children[index]; 
    } 
  
    // mark last node as leaf 
    pCrawl->isEndOfWord = true; 
} 
*/

    fn insert(sp: &mut SemanticProducer, key: &RuleMashTrieKey, trie: &mut RuleMashTrie, depth: usize, rule_id : usize) {
        type T = RuleMashTrie;
        //log
        let k = format!("{:?}", key.seq);
        let r = format!("Q{} IF ", rule_id);
        let mut evt = String::from("ADD-RULE: ");
        evt.push_str(&r);
        evt.push_str(&k);
        log56!(sp, evt);


        let mut trie1 = trie;
        let mut depth = depth;

        if T::is_null(&trie1) {
            let new_mash = RuleMashTrie::new();
            trie1.children = vec![new_mash; MAX_FLAGS];
            trie1.rule_xref.push(0);
            trie1.null_structure=false;
            //log
            let mut evt = String::from("INFO: Empty tree");
            log56!(sp, evt);
        }

        //log
        let mut evt = String::from("INFO: Begin key loop");
        log56!(sp,evt);

        let mut i = 0;
        while i < MAX_FLAGS {
            let idx : usize = T::charac(depth, key) as usize;
            if idx==0 {
                //log
                let mut evt = format!("INFO: Exit key loop;terminal key idx={} dep={}", idx, depth);
                log56!(sp, evt);

                break;//end key
            }
            //log
            let mut looper = format!("INFO: newloop of key loop idx={} dep={}", idx, depth);
            log56!(sp, looper);

            if T::is_null(&trie1.children[idx]){
                //log
                let mut intnode = format!("INFO: node null; new INT CHILD w/idx={}", idx);
                log56!(sp, intnode);

                T::set_interior_node(&mut trie1.children[idx]);
            }
            let mut movedeep = format!("INFO: move search to INT CHILD at idx={} dep={}", idx, depth);
            log56!(sp, movedeep);
            trie1 = &mut trie1.children[idx];
            depth = depth + 1;
            i = i + 1;
        }
        let mut loopdone = format!("INFO: loop done dep={}", depth);
        log56!(sp, loopdone);

        trie1.null_structure=false;
        trie1.data_structure=true;
        trie1.rule_xref.push(rule_id);
        trie1.k = *key;
        let mut loopdone2 = format!("INFO: set data node dep={} rule={}", depth, rule_id);
        log56!(sp, loopdone2);
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

    fn insert3(key: &RuleMashTrieKey, trie: &mut RuleMashTrie, depth: usize, rule_id : usize) {
        let mut sp = SemanticProducer::new();
        type T = RuleMashTrie;

        if T::is_null(trie)	{
            T::set_data_node(key,trie,rule_id);
            return;
        }

        if T::is_data(trie) {
            if trie.k == *key {
                // Key already in table push new ruleid
                trie.rule_xref.push(rule_id);
            }
            else {
                let mut trie1 = T::new();
                T::set_interior_node(&mut trie1);
                let j : usize = T::charac(depth, &trie.k) as usize;
                //trie1.children [j] = *trie;
                //move data node (contents) to properly indexed place in children array
                T::copy_node(trie, & mut trie1.children[j]);//suboptimal
                //T::insert(key, &mut trie1, depth, rule_id);
                trie1.k = key.clone();
                
                //trie=trie1;
                T::copy_node(& mut trie1,trie);//suboptimal
            }
        }
        else {
            // walk more
            let j : usize = T::charac(depth, key) as usize;
            let nextsubtree = &mut trie.children[j];
            T::insert(&mut sp, key, nextsubtree, depth + 1, rule_id);
        }
        //return trie - implicit via mut arg
        return;
    }

    //returning Tree by value is not the desire here

    fn insert2(key: &RuleMashTrieKey, trie: &mut RuleMashTrie, depth: usize, rule_id : usize) -> RuleMashTrie {
        let mut sp = SemanticProducer::new();
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
            let _subtree = T::insert(&mut sp, key, nextsubtree, depth, rule_id);
            //trie.children[j] = subtree;
            //return trie; // how to return this?
            return T::new();
        }
    }

/*
// Returns true if key presents in trie, else 
// false 
bool search(struct TrieNode *root, string key) 
{ 
    struct TrieNode *pCrawl = root; 
  
    for (int i = 0; i < key.length(); i++) 
    { 
        int index = key[i] - 'a'; 
        if (!pCrawl->children[index]) 
            return false; 
  
        pCrawl = pCrawl->children[index]; 
    } 
  
    return (pCrawl != NULL && pCrawl->isEndOfWord); 
} 
*/
    fn search (key: &RuleMashTrieKey, trie: &RuleMashTrie) -> bool {
        type T = RuleMashTrie;
        let mut depth = 1;
        let mut trie1 = trie;
        let mut i = 0;
        
        if T::is_null(trie1) {
            T::notfound(key);
            return false;        
        }

        while i < MAX_FLAGS {
            let idx : usize = T::charac(depth, key) as usize;
            if idx==0 {
                break;//end key
            }
            if T::is_null(&trie1.children[idx]){
                T::notfound(key);
                return false;
            }
            trie1 = &trie1.children[idx];
            depth = depth + 1;
            i = i + 1;
        }

        if !T::is_null(trie1) && T::is_data(trie1) {
            T::found( trie1 );
            return true;
        } 
        else {
            T::notfound(key);
            return false;
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
    fn search2 (key: &RuleMashTrieKey, trie: &RuleMashTrie){
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

    fn found (trie: &RuleMashTrie) {
        println!("Search key FOUND");
        let stuff_str = format!("{:?}", trie.k);
        println!("key: {}", stuff_str);
        let stuff_str = format!("{:?}", trie.rule_xref);
        println!("Rule-ids= {}", stuff_str);
        println!("");
    }
    fn notfound (key: &RuleMashTrieKey) {
        println!("Search key NOT FOUND:");
        let stuff_str = format!("{:?}", key);
        println!("Key= {}", stuff_str);
        println!("");
    }

} //end impl RuleMastTrie



pub fn semantic_producer_test(){
    
    let mut sp = crate::util::SemanticProducer::new();

    let mut i = 0;
    while i < 80 {
        let mut s = String::from("fred_is_dead");
        log56!(&mut sp, s);
        i = i + 1;
    }    

}

pub fn trie_test() {

    let mut sp = SemanticProducer::new();

    type T = RuleMashTrie;

    let mut t = T::new();

    let mut s = [0; MAX_FLAGS];
   
    s[0]=3;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 3;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    s[0]=3;
    s[1]=1;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 31;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    s[0]=2;
    s[1]=1;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 21;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    s[0]=2;
    s[1]=2;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 22;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    s[0]=2;
    s[1]=3;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 23;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    s[0]=1;
    s[1]=0;//end of key
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 1;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

}


pub fn trie_test4() {

    let mut sp = SemanticProducer::new();

    type T = RuleMashTrie;

    let mut t = T::new();

    let mut s = [0; MAX_FLAGS];
    s[0]=2;
    let k = RuleMashTrieKey {seq :s };

    println!("***INSERT 2***");
    let rule_id = 1;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);
    T::print_node(&t);

    T::print(&t, 1);
   
    println!("***INSERT 3***");
    s[0]=3;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 2;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    T::print(&t, 1);

    println!("*** INSERT 3,1 ***");
    s[0]=3;
    s[1]=1;
    let k = RuleMashTrieKey {seq :s };
    let rule_id = 3;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    T::print(&t, 1);
}

pub fn trie_test3() {
    let mut sp = SemanticProducer::new();
    
    type T = RuleMashTrie;

    let mut t = T::new();

    let mut s = [0; MAX_FLAGS];
    s[0]=2;
    let k = RuleMashTrieKey {seq :s };

    T::search(&k, &t);

    let mut rule_id = 1;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);


    T::search(&k, &t);

    rule_id = 2;
    T::insert(&mut sp, &k, &mut t, 1, rule_id);

    T::search(&k, &t);
    
    T::print(&t, 1);
    

    s[0]=1;
    let k2 = RuleMashTrieKey {seq :s };
    T::search(&k2, &t);

    println!("insert rule 3");
    rule_id = 3;
    T::insert(&mut sp, &k2, &mut t, 1, rule_id);
    T::search(&k2, &t);

    rule_id = 4;
    T::insert(&mut sp, &k2, &mut t, 1, rule_id);
    T::search(&k2, &t);

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
