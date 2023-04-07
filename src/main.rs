#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::Redirect;
// use rocket::response::content::Json;
use mysql::prelude::*;
use mysql::*;
use rocket::Request;
use rocket_contrib::templates::Template;
// use std::io::Read;

// use rocket_contrib::uuid::Uuid;
// use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::{Context, Tera};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
struct Employee {
    employee_id: i64,
    employee_fname: String,
    employee_lname: String,
    employee_mail: String,
    password: String,
    user_type: i64,
}
//defining form data
#[derive(FromForm, Serialize, Deserialize)]
struct MyFormData {
    username: String,
    password: String,

}

#[derive(FromForm, Serialize, Deserialize)]
struct StockFormData {
    itemname: String,
    quantity:i64,
    categoryid:i64,
}

#[derive(FromForm, Serialize, Deserialize)]
struct SalaryFormData {
    employeeid: i64,
    month:String,
}


//home get route
#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String
    }
    let context = Context {
        first_name: String::from("Ankit"),
        last_name: String::from("Rajput")
      };

    
      Template::render("home", context)
}

//user get route with parameters

#[get("/<username>/<password>")]
fn user(username:String, password:String) -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        password: String,
        user_type:String,
    }
    let context = Context {
        first_name:username,
        password:password,
        user_type:String::from("User")
      };
      Template::render("user", context)
}


//admin get route with parameters
#[get("/<username>/<password>")]
fn admin(username:String, password:String) -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        password: String,
        user_type:String
    }
    let context = Context {
        first_name:username,
        password:password,
        user_type:String::from("Admin")
      };
      Template::render("admin", context)
}

//route for render the user attendance
#[get("/yattendance?<userid>")]
fn yattendance(userid:i64) -> Template {

    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
   let result = show_attendance(&mut conn, userid);
    let mut context = Context::new();
    context.insert("content", &result);
      Template::render("show", context)
}

//route for render user leaves

#[get("/yleaves?<userid>")]
fn yleaves(userid:i64) -> Template {


    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
   let result = show_leave(&mut conn, userid);
    let mut context = Context::new();
    context.insert("content", &result);
      Template::render("leave", context)
}

//get route for user salary slip
#[get("/yslip?<userid>")]
fn yslip(userid:i64) -> Template {


    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
   let result = show_salaryslip(&mut conn, userid);
    let mut context = Context::new();
    context.insert("content", &result);
      Template::render("slip", context)
}

//get route for show stock
#[get("/stock")]
fn stock() -> Template {


    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let result = show_stock(&mut conn);
    let mut context = Context::new();
    context.insert("content", &result);
      Template::render("stock", context)
}

//get route for show employees
#[get("/employee")]
fn employee() -> Template {


    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let result = show_all(&mut conn);
    let mut context = Context::new();
    context.insert("content", &result);
      Template::render("employee", context)
}


//get route for wrong parameter
#[get("/")]
fn wrong() -> Template{
    #[derive(Serialize)]
    struct Context {
        first_name: String
    }
    let context = Context {
        first_name: String::from("Wrong Password"),
      };
    Template::render("welcome", context) 
}

//post route for insert the stock
#[post("/istock", data="<form_data>")]
fn istock(form_data: rocket::request::Form<StockFormData>) -> Template{

    let itemname = form_data.itemname.clone();
    let quantity = form_data.quantity.clone();
    let categoryid = form_data.categoryid.clone();
    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    insert_stock(&mut conn, itemname.clone(), quantity.clone(), categoryid.clone());
    let mut context = Context::new();
    let c = format!("{} is inserted in the stock",itemname);
    context.insert("content",&c );
    Template::render("response", context)
}

//post route for generating the salary slip
#[post("/gensalary", data="<form_data>")]
fn gensalary(form_data: rocket::request::Form<SalaryFormData>) -> Template{

    let employeeid = form_data.employeeid.clone();
    let month = form_data.month.clone();
    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    generate_salary(&mut conn, employeeid.clone(),month.clone());
    let mut context = Context::new();
    let c = format!("Salary is generated for Employee Id: {}",employeeid);
    context.insert("content",&c );
    Template::render("response", context)
}


//post route for taking form input
#[post("/", data = "<form_data>")]
fn login(form_data: rocket::request::Form<MyFormData>) -> Redirect {
     let username = form_data.username.clone();
     let password = form_data.password.clone();

    let mut context = Context::new();
    context.insert("username", &form_data.username);

    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    //creating a connection
    let mut conn = pool.get_conn().unwrap();
    let uname:String = username.clone();
    let v:Vec<String> = signin(&mut conn, &username);
    let pass:String = v[0].clone();
    let fname:String = v[3].clone();
    let uid:String = v[4].clone();
    if password == pass{
        if uid == "1".to_string(){
            let url = format!("/admin/{}/{}",fname,password);
            Redirect::to(url)
        }
        else{
            let url = format!("/user/{}/{}",fname,password);
            Redirect::to(url)
        }
    }
    else{
        Redirect::to("/wrong")
    }
}


#[post("/", data = "<form_data>")]
fn attendance(form_data: rocket::request::Form<MyFormData>) -> Template {
     let userid = form_data.username.clone();
     let password = form_data.password.clone();
     let n: i64 = userid.trim().parse().unwrap();
    let mut context = Context::new();
    context.insert("password", &form_data.password);

    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    //creating a connection
    let mut conn = pool.get_conn().unwrap();
    // let uname:String = username.clone();
    // let v:Vec<String> = signin(&mut conn, &username);
    mark_attendance(&mut conn, n);
    Template::render("attendance", context)
}

//route for applying leave
#[post("/", data = "<form_data>")]
fn apleave(form_data: rocket::request::Form<MyFormData>) -> Template {
     let d:String = form_data.username.clone();
     let i = form_data.password.clone();
     let lday: i64 = d.trim().parse().unwrap();
    
    let lid: i64 = i.trim().parse().unwrap();
    let mut context = Context::new();
    let cntnt = format!("You have successfully applied for {} days leave",&lday);
    context.insert("content",&cntnt);

    let url = "mysql://root:7860$Ankit@localhost:3306/mcn";
    let pool = Pool::new(url).unwrap();
    //creating a connection
    let mut conn = pool.get_conn().unwrap();
    leave(&mut conn, lid, lday);
    Template::render("response", context)
}

//route for 404 rsponse
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}


//main function
fn main() {
    rocket::ignite()
  .mount("/login", routes![login])
  .register(catchers![not_found])
  .mount("/", routes![index,yattendance,yleaves,yslip,istock, stock,employee,gensalary])
  .mount("/user", routes![user])
  .mount("/admin", routes![admin])
  .mount("/wrong", routes![wrong])
  .mount("/attendance", routes![attendance])
  .mount("/apleave", routes![apleave])
  .attach(Template::fairing())
  .launch();


//   let url = "mysql://your_username:password@localhost:3306/db_name";
//   let pool = Pool::new(url).unwrap();
//   //creating a connection
//   let mut conn = pool.get_conn().unwrap();
}

fn signin(cn: &mut PooledConn, mail: &String) ->Vec<String> {
    let mut result:Vec<String> = Vec::new();
    let y=format!("select EmployeeID, EmployeeFirstName, EmployeeLastName, EmployeeEmail, password, UserTypeId from employee where EmployeeEmail= \"{}\"",mail);
    let res = cn
        .query_map(
            y,
            |(
                employee_id,
                employee_fname,
                employee_lname,
                employee_mail,
                password,
                user_type,
            )| Employee {
                employee_id: employee_id,
                employee_fname: employee_fname,
                employee_lname: employee_lname,
                employee_mail: employee_mail,
                password: password,
                user_type: user_type,
            },
        )
        .expect("Query failed.");
    let mut pass: String = String::new();
    let mut mail: String = String::new();
    let mut esid:String = String::new();
    let mut name: String = String::new();
    let mut utid: String  = String::new();
    for r in res {
        pass = r.password;
        mail = r.employee_mail;
        esid = r.employee_id.to_string();
        name = r.employee_fname;
        utid = r.user_type.to_string();
        result.push(pass);
        result.push(mail);
        result.push(esid);
        result.push(name);
        result.push(utid);
    }
    result
}
fn mark_attendance( cn:&mut PooledConn , id:i64)
{
cn.exec_drop(
   "insert into attendance (EntryDate, IsPresent, employeeId) values (now(), :ispresent, :empid)",
   params! {
       "ispresent" => "true",
       "empid" => id
       
   },
).unwrap();
//println!("Last generated key: {}", conn.last_insert_id());
}

fn leave( cn:&mut PooledConn, lid:i64,lday:i64)
    {
    cn.exec_drop(
        "INSERT INTO leavetable ( Month, NoOfDays, employeeId) VALUES ( now(), :days, :empid)",
        params! {
            "empid" => lid,
            "days" => lday,
            
        },
    ).unwrap();
    }


//show leaves
fn show_leave( cn:&mut PooledConn, eid:i64) -> Vec<(i64,i64,String)>
    {
        // let mut leave =0;
        let lqr:String = format!("select leaveId, NoOfDays,Month from leavetable where employeeId = {}", eid);
        let res:Vec<(i64,i64,String)> = cn.query(lqr).unwrap();
        // for r in res{
        //     println!("Number of Days:{}| Date of leave:{}", r.0,r.1);
        //     // leave= r.0;
        // }
        res
    }

//show attendance
fn show_attendance(cn:&mut PooledConn, id:i64) -> Vec<(i64, i64, String, String)>
    {
        let qr = format!("select * from attendance where employeeId = {}",id);
        let res:Vec<(i64, i64, String, String)> = cn.query(qr).unwrap();
    
    // for r in res {
    //     println!("Attendance ID:{} |Employee ID:{} |Date:{} |Is Present:{}", r.0, r.1, r.2,r.3);
    // }
    res
    }


//function for show salary slip
fn show_salaryslip( cn:&mut PooledConn, eid:i64) ->Vec<(i64,i64,String,i64)>
{
   
    let lqr:String = format!("select * from salaryslip where employeeId = {}", eid);
    let res:Vec<(i64,i64,String,i64)> = cn.query(lqr).unwrap();
    // for r in res{
    //     println!("Salary Slip Id:{} | Employee Id:{}| Month:{} | Paid Salary:{}", r.0,r.1,r.2,r.3);
     
    // }
    res
}

//function for stock insertion
fn insert_stock(cn:&mut PooledConn,itemname:String,quantity:i64,categoryid:i64)
{
    let query="INSERT INTO stock (ItemName,quantity,categoryId) values (:itemname, :quantity, :categoryid)";
    let params=params!{"itemname"=>itemname, "quantity"=>quantity,"categoryid"=>categoryid,};
    cn.exec_drop(query,params).unwrap();
}

//function for display all stocks

fn show_stock(cn:&mut PooledConn) -> Vec<(i64,String,i64, String, i64)>
    {
        let qr = format!("select ItemId, ItemName, stock.categoryId, categoryName, quantity from stock inner join category on stock.categoryId = category.categoryId");
        let res:Vec<(i64,String,i64, String, i64)> = cn.query(qr).unwrap();
    
    // for r in res {
    //     println!("Item ID:{} | Item Name:{} | Category Id:{} | Category Name:{} | Quantity:{}", r.0, r.1, r.2,r.3, r.4);
    // }
    res
    }


//function to display all employees

fn show_all(cn:&mut PooledConn) -> Vec<(i64, String, String,String,String,String,i64,String,i64,i64)>
   {
    let res:Vec<(i64, String, String,String,String,String,i64,String,i64,i64)> = cn.query("select * from employee")
    .unwrap();
    res
   }
//function for generating the salary
fn generate_salary( cn:&mut PooledConn, id:i64, month:String)
     {
        
        
       
        let mut salary = 0;
        //let sqr:String = format!("select salary from employee where employeeId = {id}");
        let sqr:String = format!("select salary from employee where employeeId = {}",&id);
        let res:Vec<i64> = cn.query(sqr).unwrap();
        for r in res{
            println!("salary:{}", r);
            salary = r;
        }
        let mut leave =0;
        let lqr:String = format!("select NoOfDays from leavetable where employeeId = {}", &id);
       // let lqr:String = format!("select NoOfDays from leavetable where employeeId = {id}");
        let res:Vec<i64> = cn.query(lqr).unwrap();
        for r in res{
            // println!("Number of Days:{}", r);
            leave += r;
        }
        println!("total leaves:{}",leave);
        let deduction = (salary*leave)/30;
        println!("deduction: {}", deduction);
        let  amount = &salary-(deduction);
        
        let gross_salary = &amount;
        println!("Total Salary: {}",gross_salary);


             let stmt = cn.prep("INSERT INTO SalarySlip ( EmployeeID, Month,  PaidSalary) VALUES (:eid, :month,:ps)")
             .unwrap();   

             cn.exec_drop(&stmt, params! {
                "eid" => id,
                "month" => month,
                "ps" => gross_salary,
            }).unwrap();
    }