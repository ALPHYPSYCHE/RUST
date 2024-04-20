pub mod pizza_order{
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: "Thin Crust Dough".to_string(),
                cheese: "Marinara Cheese".to_string(),
                topping: topping.to_string(),
            }
        }
    }

    pub mod helip_customer{
        pub fn seat_at_table(){
            println!("Hello, welcome to the seat at the table!");
        }
        pub fn take_order(){
            super::helip_customer::seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("Mozzarella");
            super::helip_customer::serve_customer(cust_pizza);
        }

        pub fn serve_customer(cust_pizza: super::Pizza){
            println!("Here is your pizza: ");
            println!("The customer is served a regular pizza with {}", cust_pizza.topping);
        }
    
    }
}

pub fn order_food(){
    pizza_order::helip_customer::take_order();
}
