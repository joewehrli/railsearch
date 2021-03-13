#![allow(dead_code)]


use crate::util::SemanticProducer;


#[cfg(foo)]
const MAX_FLAGS: usize = 10;
const MAX_FLAGS: usize = 4;


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
    
    let mut sp = SemanticProducer::new();

    let mut i = 0;
    while i < 80 {
        let mut s = String::from("fred_is_dead");
        log56!(&mut sp, s);
        i = i + 1;
    }
}
