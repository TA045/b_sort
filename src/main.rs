use rand::Rng;

fn main() {

  let mut vec:Vec<i32> = Vec::new();

  for _n in 1..=1_000 {
    let r = rand::thread_rng().gen_range(0..10);
    vec.push(r);
  }

  //vec.sort();

  //println!("{:?}",vec);  

  let mut _ar:[(i32,i32); 1_000] = [(0,0); 1_000];

  let mut zc = 0;
 
  let mut _z = 0;


  for _m in 0..=999 {

  let _n = vec.pop();

    if let Some(v) = _n {

      if v == 0 {

        zc+=1

      }
      
      _z = _ar[v as usize].1;

      if v == _ar[v as usize].0 {

        _z += 1;

      }

      _ar[v as usize] = (v as i32, _z) ;

    }

  }


  //println!("{:?}", _ar);  

  // sort

  let mut ves =  Vec::new();

  for _o in 0..=999 {


     while zc > 0 {

       ves.push(0);
       
       zc -=1;

     } 

     if _ar[_o as usize].0 > 0 {

       ves.push(_ar[_o as usize].0);

       let mut _t = _ar[_o as usize].1;

       while _t > 0 {

         ves.push(_ar[_o as usize].0);

         _t -= 1;

       }

     }

  }

  //println!("{:?}", ves);  

}
