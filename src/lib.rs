#![allow(dead_code)]


#[path = "util.rs"]
mod util;

#[path = "ruletree.rs"]
mod ruletree;


use crate::util::SemanticProducer;
use ruletree::RuleMashTrie;
use ruletree::RuleMashTrieKey;
use ruletree::MAX_SYM;
use ruletree::MAX_INFO;


pub fn run_trie() {

    let mut sp = SemanticProducer::new();

    type T = RuleMashTrie;

    let mut t = T::new();

    let mut s = [0; MAX_SYM];

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

    //test eval against
    println!("\nmatch bits=[t,t,f,f,f,...");
    let mut bits = [false; MAX_INFO];
    bits[0]=true;
    bits[1]=true;
    T::eval(&mut sp, &t, &bits);

}