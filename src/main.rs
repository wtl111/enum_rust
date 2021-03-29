trait LampTime{
    fn getLampTime(&self)-> i32;
}
enum StreetLamp{
    green,
    red,
    yellow,
}

impl LampTime for StreetLamp{
    fn getLampTime(&self) -> i32{
      match *self{
            StreetLamp::green => 60,
            StreetLamp::red => 70,
            _=> 80,
        }
    } 
}


//计算集合的和
fn calculationSum(vec: &Vec<u32>)-> Option<u32> {
    let mut sums: u32=0;
    for i in vec{
       match  sums.checked_add(*i) {
            Some(s) => {sums=sums+s;}
            None => {return None;}  
       };
    }
    Some(sums)
}

trait CalculateArea{
    fn calculat(&self) ;
}

impl CalculateArea for triangle{
    fn calculat(&self) {
       let areas= self.bottom*self.high;
       println!("三角形面积 {}",areas);
    }
}


impl CalculateArea for square{
    fn calculat(&self) {
       let areas= self.length*self.length;
       println!("正方形面积 {}",areas);
    }
}

struct triangle{
    bottom: u32,
    high: u32
}

struct square{
    length: u32
}

fn getArea<T:CalculateArea>(graphics: &T){
    graphics.calculat();
}

fn main() {
    //红绿灯
    let reds= StreetLamp::red;
    let times=reds.getLampTime();
    println!("red ={}",times);

    //计算集合的和
    let  v=vec![1,2,3];
    calculationSum(&v);

    //三角形面积
    let triangle=triangle{bottom: 10,high: 10 };
    getArea(&triangle);

     //三角形面积
     let square=square{length: 5 };
     getArea(&square);
}








    
