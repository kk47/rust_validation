fn main() {

	// if test
	let a = 11;
	if a == 10
	{
		println!("a equal to 1");	
	} else if a > 10
	{
		println!("a is greater than 10");
	} else {
		println!("a is less than 10");
	}
	
	// loop test
	let mut cnt = 0;
	loop{
		println!("Hello, world!");	
		if cnt > 10
		{
			break;
		}
		cnt += 1;
	}

	// for test
	let mut res;
	for i in 1..10 {
		res = 2 * i;
		println!("2*{}={} ", i, res);	
	}
	
	let fruits = ["mango", "apple", "banana", "litchi", "watermelon"];
	for a in fruits.iter()
	{
		print!("{} ", a);
	}
	print!("\n");

	// while test
	let mut i = 1;
	while i <= 10
	{
		print!("{}", i);
		print!(" ");
		i += 1;
	}
	print!("\n");
	let arr = [10,20,30,40,50,60];
	let mut j = 0;
	while j < 6
	{
		print!("{}", arr[j]);
		print!(" ");
		j += 1;
	}

	let x = gives_ownership();
	println!("value of x is:{}", x);

	// mut ref test
	let mut z = 1i32;
	value_changed(&mut z);
	println!("Before modifying 1 after modifying is:{}", z);

    // struct test
    let triangle= Triangle{base:20.0,height:30.0};  
    println!("Area of a right angled triangle is {}", area(&triangle));

    println!("Area of a rectangle is {}", triangle.area_rectangle());

    // enum test
    let n = Employee::Name("Hema".to_string());  
    let i = Employee::Id(2);  
    let p = Employee::Profile("Computer Engineer".to_string());  
    println!(" {:?} {:?} {:?}", n,i,p);

    // if let test
    let a=Some(3);  
    if let Some(3)=a{  
     println!("three");  
    }

}

#[derive(Debug)]
enum Employee {  
    Name(String),  
    Id(i32),  
    Profile(String),  
}

struct Triangle  
{  
    base:f64,  
    height:f64,  
}  

impl Triangle  
{  
    fn area_rectangle(&self)->f64  
    {  
        self.base * self.height
    }  
}

fn area(t:&Triangle)->f64  
{  
    0.5 * t.base * t.height  
}

fn gives_ownership()->u32
{
	let y = 100;
	y
}

fn value_changed(y:&mut i32)
{
	*y = 9i32;
}
