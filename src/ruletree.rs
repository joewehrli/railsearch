#![allow(dead_code)]

use std::convert::TryInto;
use crate::util::SemanticProducer;


const MAX_FLAGS: usize = 10;


macro_rules! log56 {
    ( $( $x:expr, $y:expr ), *) => {
        {
            $(
                SemanticProducer::pad56(&mut $y);
                SemanticProducer::produce56($x, $y.as_bytes());
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

#[derive(Debug, Clone, Copy)]
pub struct RuleMashTrieKey{
    pub seq :[i8; MAX_FLAGS]
}

// a trie like structure that mashes together business rules for mashed evaluation
//MAX_FLAGS = alphabet of size 10; 10 means rules <1,10> with index <0,9>

#[derive(Clone)]
pub struct RuleMashTrie {
    null_structure : bool, 
    data_structure : bool, 
    rule_xref : Vec<usize>, 
    k : RuleMashTrieKey,
    children : Vec<RuleMashTrie>
}

impl RuleMashTrie {

    pub fn get_child_vec(t: &RuleMashTrie) -> &Vec<RuleMashTrie>{
        return &t.children;
    }

    pub fn new() -> Self {

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

    pub fn set_interior_node(trie: &mut RuleMashTrie){
        trie.null_structure=false;
        trie.data_structure = false;
        let new_mash = RuleMashTrie::new();
        trie.children = vec![new_mash; MAX_FLAGS];
    }

    

    pub fn dump_node1 (trie: &RuleMashTrie){
        type T = RuleMashTrie;

        println!("BGN dump_node1()");

        println!("null_structure= {}", trie.null_structure);
        println!("data_structure= {}", trie.data_structure);

        let stuff_str = format!("{:?}", trie.k);
        println!("key: {}", stuff_str);

        let stuff_str = format!("{:?}", trie.rule_xref);
        println!("Rule-ids= {}", stuff_str);

        println!("END dump_node1()");
    }

    pub fn dump_node (trie: &RuleMashTrie){
        type T = RuleMashTrie;

        T::dump_node1(&trie);
        println!("\ndirect children:");
        let mut i = 0;
        while i < MAX_FLAGS {
            let trie1 = &trie.children[i];
            println!("\nindex= {}", i);
            T::dump_node1(&trie1);
            i = i + 1;
        }
    }

    //export rules to stdout
    pub fn dump_rules (trie: &RuleMashTrie, depth: usize){
        type T = RuleMashTrie;

        if T::is_null(trie) {
            if depth==1 {
                println!("No rules dep={}",depth);
            }
            return;
        }

        if T::is_data(trie) {
            let k_part = format!("{:?}", trie.k.seq);
            let q_part = format!("{:?}", trie.rule_xref);
            println!("IF{} THEN Q{}", k_part, q_part);
        }
        
        let mut i = 0;
        while i < MAX_FLAGS {
            T::dump_rules( &trie.children[i], depth + 1 );
            i = i + 1;
        }

        return;
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
    pub fn print (trie: &RuleMashTrie, depth: usize){
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
    pub fn insert(sp: &mut SemanticProducer, key: &RuleMashTrieKey, trie: &mut RuleMashTrie, depth: usize, rule_id : usize) {
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
        let mut last_idx = 0;
        while i < MAX_FLAGS {
            let idx = T::charac(depth, key);
            if idx==0 {
                //log
                let mut evt = format!("INFO: Exit key loop;terminal key idx={} dep={}", idx, depth);
                log56!(sp, evt);

                break;//end key
            }
            //log
            let mut looper = format!("INFO: newloop of key loop idx={} dep={}", idx, depth);
            log56!(sp, looper);

            if T::is_null(&trie1.children[T::ncpos(idx)]){
                //log
                let mut intnode = format!("INFO: node null; new INT CHILD w/idx={}", idx);
                log56!(sp, intnode);

                T::set_interior_node(&mut trie1.children[T::ncpos(idx)]);
            }
            let mut movedeep = format!("INFO: move search to INT CHILD at idx={} dep={}", idx, depth);
            log56!(sp, movedeep);
            trie1 = &mut trie1.children[T::ncpos(idx)];
            depth = depth + 1;
            i = i + 1;
            last_idx = idx;
        }
        let mut loopdone = format!("INFO: loop done dep={} last_idx={} i={}", depth, last_idx, i);
        log56!(sp, loopdone);

        trie1.null_structure=false;
        trie1.data_structure=true;
        trie1.rule_xref.push(rule_id);
        trie1.k = *key;
        let mut loopdone2 = format!("INFO: set data node dep={} rule={}", depth, rule_id);
        log56!(sp, loopdone2);
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
    pub fn search (key: &RuleMashTrieKey, trie: &RuleMashTrie) -> bool {
        type T = RuleMashTrie;
        let mut depth = 1;
        let mut trie1 = trie;
        let mut i = 0;
        
        if T::is_null(trie1) {
            T::notfound(key);
            return false;        
        }

        while i < MAX_FLAGS {
            let idx = T::charac(depth, key);
            if idx==0 {
                break;//end key
            }
            if T::is_null(&trie1.children[T::ncpos(idx)]){
                T::notfound(key);
                return false;
            }
            trie1 = &trie1.children[T::ncpos(idx)];
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

    // take a seq [4 3 2 1] and for dep x gives [d-1] from seq
    // at depth 1 look at [0]=4
    // at depth 2 look at [1]=3
    // at depth 3 look at [2]=2
    // at depth 4 look at [3]=1
    pub fn charac(depth: usize, key: &RuleMashTrieKey) -> i8{
        assert!(depth>0);
        let idx = depth-1; 
        let val = key.seq[idx];
        return val;
    }

    //map seq val to an array pos
    //supports only positive 1,2,3... no negations terms
    fn cpos(i:i8)-> usize {
        let i_max_flags : i8 = MAX_FLAGS.try_into().unwrap();
        assert!(i>0);
        let r = i - 1;
        assert!(r>=0 && r<i_max_flags);
        let r0 : usize = r.try_into().unwrap();
        r0
    }

    //map seq val to an array pos
    //supports negative and positive -3,-2,-1, 1,2,3... handles negations terms
    pub fn ncpos(seqval:i8)-> usize {
        let i = seqval;
        let i_max_flags : i8 = MAX_FLAGS.try_into().unwrap();
        assert!(i_max_flags % 2 == 0); // MAX_FLAGS must be even
        assert!(i>=(-i_max_flags/2));  // must be in alphabet
        assert!(i<=(i_max_flags/2));  // must be in alphabet
        assert!(i != 0);            // must be in alphabet
        let r;
        let mid = i_max_flags/2;
        if i < 0 {
            r = i + mid;
        }
        else { //i > 0
            r = i + mid - 1;
        }
        assert!(r>=0 && r<i_max_flags);
        let r0 : usize = r.try_into().unwrap();
        r0
    }

    //unitialized node
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
    
    let mut sp = SemanticProducer::new();

    let mut i = 0;
    while i < 80 {
        let mut s = String::from("fred_is_dead");
        log56!(&mut sp, s);
        i = i + 1;
    }
}


#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn ncpos_test() {
        //works with usized = 6
        /*
        type T = RuleMashTrie;
        let r = T::ncpos(-5);
        assert!(r==0);
        let r = T::ncpos(-4);
        assert!(r==1);
        let r = T::ncpos(-3);
        assert!(r==2);
        let r = T::ncpos(3);
        assert!(r==3);
        let r = T::ncpos(4);
        assert!(r==4);
        let r = T::ncpos(5);
        assert!(r==5);
*/
        //works with usized = 10
        type T = RuleMashTrie;
        let r = T::ncpos(-5);
        assert!(r==0);
        let r = T::ncpos(-4);
        assert!(r==1);
        let r = T::ncpos(-3);
        assert!(r==2);
        let r = T::ncpos(3);
        assert!(r==7);
        let r = T::ncpos(4);
        assert!(r==8);
        let r = T::ncpos(5);
        assert!(r==9);
    }

    #[test]
    pub fn trie_test() {

        let mut sp = SemanticProducer::new();

        type T = RuleMashTrie;
    
        let mut t = T::new();
    
        let mut s = [0; MAX_FLAGS];
    
        s[0]=3;
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 3;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
        let rule_id = 3000;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
        println!("\n******ROOT*******\n");
        T::dump_node(&t);
        
    
        s[0]=3;
        s[1]=1;
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 31;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
        let rule_id = 3100;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
    
        println!("\n******ROOT POST INS 3,1*******\n");
        T::dump_node(&t);
    
    
        s[0]=3;
        s[1]=-1;
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 310000000;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
        let rule_id = 3100000002;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
    
        println!("\n******ROOT POST INS 3,-1*******\n");
        T::dump_node(&t);
    
    
    
        println!("\n******CHILD 3 POST INS 3,1*******\n");
        //let t2=&t.children[3];
        let t2=T::get_child_vec(&t);
        s[0]=3;
        s[1]=0;
        let k = RuleMashTrieKey {seq :s };
        let i = T::charac(1, &k);
        let i2 = T::ncpos(i);
        //T::dump_node(&t2[3-1]);
        T::dump_node(&t2[i2]);
    
    
    
    
    
        println!("\n******SEARCH 3*******\n");
        s[0]=3;
        s[1]=0;
    
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        println!("\n******SEARCH 3,1*******\n");
        s[0]=3;
        s[1]=1;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr); 
    
        println!("\n******SEARCH 3,-1*******\n");
        s[0]=3;
        s[1]=-1;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr); 
    
    
    
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
    
        s[0]=2;
        s[1]=-3;
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 230000000;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
    
    
        s[0]=1;
        s[1]=0;//end of key
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 1;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
    
        s[0]=1;
        s[1]=3;
        let k = RuleMashTrieKey {seq :s };
        let rule_id = 13;
        T::insert(&mut sp, &k, &mut t, 1, rule_id);
    
        // dump root
        println!("\n******ROOT POST INS 2,1 2,2 2,3 2,-3 1,3 *******\n");
        T::dump_node(&t);
    
        println!("\n******CHILD 2 POST INS 2,1 2,2 2,3 2,-3 1,3*******\n");
        //let t2=&t.children[2];
        let t2=T::get_child_vec(&t);
        s[0]=2;
        s[1]=0;
        let k = RuleMashTrieKey {seq :s };
        let i = T::charac(1, &k);
        let i2 = T::ncpos(i);
        //T::dump_node(&t2[2-1]);
        T::dump_node(&t2[i2]);
    
        println!("\n******SEARCH 2,1*******\n");
        s[0]=2;
        s[1]=1;
    
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        println!("\n******SEARCH 2,2*******\n");
        s[0]=2;
        s[1]=2;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        println!("\n******SEARCH 2,3*******\n");
        s[0]=2;
        s[1]=3;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        println!("\n******SEARCH 2,-3*******\n");
        s[0]=2;
        s[1]=-3;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
    
        println!("\n******SEARCH 1*******\n");
        s[0]=1;
        s[1]=0;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        println!("\n******SEARCH 1,3*******\n");
        s[0]=1;
        s[1]=3;
        let k = RuleMashTrieKey {seq :s };
        let sr = T::search(&k,&t);
        assert!(sr);
    
        //export keys and rules
        println!("\nrule dump");
        T::dump_rules(&t,1);
    }
}