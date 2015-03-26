use std::cmp::Ordering;
use std::rand::{thread_rng, Rng};

fn cmp(a: u64, b: u64) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn substr(content: &str, i: u64, len: u64) -> String {
    let v = content.to_string();
    let iter: Vec<_> = v.as_bytes().iter().skip(i).take(len).collect();
    let mut vv = Vec::with_capacity(iter.len());
    for i in 0..(iter.len()) {          
      vv.push(*iter[i]);          
    }    
    let s = String::from_utf8(vv).unwrap();   
    return s
}
  

#[test]
fn random_acess_read_write() {

    let mut size = 0;
    let broken_data = Vec::<(u64,String)>::new();
    let extra = "amended".to_string();
    let mut piece = Vec::<(u64,String)>::new(); 
    let mut broken_data = Vec::<(u64,String)>::new();
    let mut i += 0;
    let content = random_string[4096*5];
    let mut kdatasize = content.len();

    loop { 
          match cmp((kdatasize - i) as u64, ((4096*5) as u64) {
               Ordering::Less => {let size = kdatasize - i;}
                   _ => size = rand::random::<f32>() % (4096*5);
                 }
          piece.push((i, substr(content, i as u64, size as u64));  
          broken_data.push(piece); 
          i += size;
          if i >= kdatasize == 0 { break; }
    }
    let tupl: (u64,String);
    
    {
      let mut last_piece = broken_data.iter().enumerate().last().unwrap();
      let tupl = *last_piece.1;
    }

    let mut rng = thread_rng();
    rng.shuffle(&mut broken_data);

    let tupl2: (u64, String);

    {
      let mut overlap = broken_data.iter().enumerate().last().unwrap();
      let tupl2 = *overlap.1;
    }

    //let mut overlap = broken_data.iter().enumerate().last().unwrap();
    if (tupl == tupl2) {
      broken_data.iter().next();
    }
    let post_overlap = Vec::<(u64,String)>::new();
    post_overlap.push(broken_data.iter().next().unwrap().0,broken_data.iter().next().unwrap().1);
    let post_position = (broken_data.iter().next().unwrap().0 + broken_data.iter().next().unwrap().1.to_string().len());
    for (x,item) in broken_data {
      assert_eq!(se.write(broken_data[x]))
    }

          

       
                               
           