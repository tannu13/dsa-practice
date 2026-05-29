use std::collections::HashMap;

fn add_qty(prices: &mut HashMap<i32, i32>, price: i32, qty: i32) {
    prices
        .entry(price)
        .and_modify(|existing_qty| *existing_qty += qty)
        .or_insert(qty);
}

fn remove_qty(prices: &mut HashMap<i32, i32>, price: i32, qty: i32) {
    let existing_qty = prices.get(&price).cloned().unwrap_or(0);
    if existing_qty > qty {
        prices.insert(price, existing_qty - qty);
    } else {
        prices.remove(&price);
    }
}

fn get_lowest_price(prices: &mut HashMap<i32, i32>) -> i32 {
    let mut lowest_price = i32::MAX;
    for (&price, _qty) in prices {
        if price < lowest_price {
            lowest_price = price;
        }
    }

    return lowest_price;
}

fn get_highest_price(prices: &mut HashMap<i32, i32>) -> i32 {
    let mut highest_price = i32::MIN;
    for (&price, _qty) in prices {
        if price > highest_price {
            highest_price = price;
        }
    }

    return highest_price;
}

fn main() {
    let mut prices = HashMap::new();
    println!("{:?}", prices);
    add_qty(&mut prices, 9, 4);
    add_qty(&mut prices, 9, 10);
    add_qty(&mut prices, 9, 1);
    add_qty(&mut prices, 16, 5);
    add_qty(&mut prices, 14, 9);
    remove_qty(&mut prices, 9, 5);
    add_qty(&mut prices, 8, 2);
    println!(
        "Lowest: {} \nHighest: {}",
        get_lowest_price(&mut prices),
        get_highest_price(&mut prices)
    );
    println!("{:?}", prices);
}
