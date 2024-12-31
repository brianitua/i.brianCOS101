fn main() {

    let commissioners = vec![
        "Aigboqun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("{} | {} | {}", "Name", "Ministry", "Geopolitical Zone");
    println!("{} {} {}", "", "", "");

    for i in 0..commissioners.len() {
        println!("{} - {} - {}", commissioners[i], ministries[i], zones[i]);
    }
}
