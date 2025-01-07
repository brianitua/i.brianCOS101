struct Laptop {
    price: u32,
}

impl Laptop {
    fn total_cost(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    
    
    let toshiba = Laptop {
        price: 550000,
};
    let hp = Laptop {
        price: 650000,
};
    let dell = Laptop {
        price: 850000,
};
    let ibm = Laptop {

        price: 755000,
};

     

    let total_cost = hp.total_cost()
        + ibm.total_cost()
        + toshiba.total_cost()
        + dell.total_cost();

    println!("Total cost: N{}", total_cost);
}
