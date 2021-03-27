use std::fs::File;
use std::io::Write;


// simple program semantic event producer
pub struct SemanticProducer {
    file_path : String,
    file_ref : File
}


impl SemanticProducer {
    pub fn new() -> Self {
        let loc = "semantic_events56.txt"; //56 is 64 bytes fixed record length
        let f = File::create(loc);
        assert!( f.is_ok() );

        let fr = f.unwrap();

        return SemanticProducer {
            file_path: loc.to_owned(),
            file_ref : fr
        };
    }

    //mutables arg - avoid copy
    //56 data goes to 64 bytes fixed record length
    pub fn pad56(event_user_data : &mut String) { 
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len1={}",byte_len);
        assert!(byte_len <= 56); // in bytes not char count
        let i = byte_len;

        let s = String::from(" ");
        let add = 64 - i - 1;
        let s2 = s.repeat(add);
        event_user_data.push_str(&s2);
        
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len2={}",byte_len);
        assert!(byte_len == 63); // in bytes not char count

        event_user_data.push('\n');//linux only - windows needs \r\n
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len3={}",byte_len);
        assert!(byte_len == 64); // in bytes not char count
    }

    pub fn produce56(sp : &mut SemanticProducer, event_data : &[u8])  {
        let byte_len = event_data.len();
        assert!(byte_len == 64); // in bytes not char count
        //let mut f = sp.file_ref;
        //f.write_all(event_data);
        sp.file_ref.write_all(event_data);
    }

    //mutables arg - avoid copy
    //120 data goes to 128 bytes fixed record length
    pub fn pad120(event_user_data : &mut String) { 
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len1={}",byte_len);
        assert!(byte_len <= 120); // in bytes not char count
        let i = byte_len;

        let s = String::from(" ");
        let add = 128 - i - 1;
        let s2 = s.repeat(add);
        event_user_data.push_str(&s2);
        
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len2={}",byte_len);
        assert!(byte_len == 127); // in bytes not char count

        event_user_data.push('\n');//linux only - windows needs \r\n
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len3={}",byte_len);
        assert!(byte_len == 128); // in bytes not char count
    }

    pub fn produce120(sp : &mut SemanticProducer, event_data : &[u8])  {
        let byte_len = event_data.len();
        assert!(byte_len == 128); // in bytes not char count
        //let mut f = sp.file_ref;
        //f.write_all(event_data);
        sp.file_ref.write_all(event_data);
    }


    //mutables arg - avoid copy
    //248 data goes to 256 bytes fixed record length
    pub fn pad248(event_user_data : &mut String) { 
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len1={}",byte_len);
        assert!(byte_len <= 248); // in bytes not char count
        let i = byte_len;

        let s = String::from(" ");
        let add = 256 - i - 1;
        let s2 = s.repeat(add);
        event_user_data.push_str(&s2);
        
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len2={}",byte_len);
        assert!(byte_len == 255); // in bytes not char count

        event_user_data.push('\n');//linux only - windows needs \r\n
        let byte_len = event_user_data.len(); // byte len - not char count
        //println!("byte_len3={}",byte_len);
        assert!(byte_len == 256); // in bytes not char count
    }

    pub fn produce248(sp : &mut SemanticProducer, event_data : &[u8])  {
        let byte_len = event_data.len();
        assert!(byte_len == 256); // in bytes not char count
        //let mut f = sp.file_ref;
        //f.write_all(event_data);
        sp.file_ref.write_all(event_data);
    }


/*
0 	1 	
1 	2 	
2 	4 
3 	8 	
4 	16 
5 	32 	
6 	64 	
7 	128
8 	256
9 	512
10 	1024
11 	2048
12  4098
*/
}
