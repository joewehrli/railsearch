#![allow(dead_code)]

#[cfg(foo)]
const MAX_RECORDS: usize = 800_000_000;

// 1 Billion calls to test/test2 with 100,000 x 10,000
const MAX_RECORDS: usize = 100_000;
const MAX_QUERIES: usize = 10_000;

const MAX_FLAGS: usize = 10;


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
