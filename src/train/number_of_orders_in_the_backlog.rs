// https://leetcode-cn.com/problems/number-of-orders-in-the-backlog/
// https://ipotato.me/article/59

/// binary heap 应用T类型

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// struct 按价格排序
// 卖单队列, 使用最小堆, 价格低的排前面
// 买单队列, 使用最大堆, 价格高的排前面
// 然后匹配两个堆的数据

// Buy = 0
// Sell = 1

// PartialOrd 要求你的类型实现 PartialEq
// Ord 要求你的类型实现 PartialOrd 和 Eq（因此 PartialEq 也需要被实现）

#[derive(Debug, Eq, Clone, Copy)]
struct MinOrder {
    price: i32,
    amount: i32,
}

impl PartialEq for MinOrder {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Ord for MinOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for MinOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(&self))
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct MaxOrder {
    price: i32,
    amount: i32,
}

impl PartialEq for MaxOrder {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Ord for MaxOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for MaxOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn get_number_of_backlog_orders(_orders: Vec<Vec<i32>>) -> i32 {
    let mut buy_heap: BinaryHeap<MaxOrder> = BinaryHeap::new();
    let mut sell_heap: BinaryHeap<MinOrder> = BinaryHeap::new();

    for t in _orders.iter() {
        // println!("{} {} {}", t[0], t[1], t[2]);


        match t[2] {
            // buy order
            0 => {
                let mut buy_order = MaxOrder { price: t[0], amount: t[1] };

                loop {
                    // 从卖单里check
                    let top_sell_order = sell_heap.peek().cloned();

                    // 卖单队列为空
                    if top_sell_order.is_none() {
                        buy_heap.push(buy_order);
                        break;
                    }

                    let tso = top_sell_order.unwrap();
                    // 买单价格比卖单低, 追加买单队列
                    if buy_order.price < tso.price {
                        buy_heap.push(buy_order);
                        break;
                    }

                    // 买单价格比卖单高,可以成交
                    if buy_order.price >= tso.price {
                        // 完全成交
                        if buy_order.amount == tso.amount {
                            // 消除卖单, 买单不进队列
                            sell_heap.pop();
                            break;
                        }
                        // 卖单多
                        else if buy_order.amount < tso.amount {
                            // 消除部分卖单, 买单不进队列
                            sell_heap.pop();
                            sell_heap.push(MinOrder { price: tso.price, amount: tso.amount - buy_order.amount });
                            break;
                        }
                        // 买单多
                        else if buy_order.amount > tso.amount {
                            // 匹配不完的买单继续和顶部卖单匹配
                            buy_order = MaxOrder { price: buy_order.price, amount: buy_order.amount - tso.amount };
                            sell_heap.pop();
                        }
                    }
                } // end of loop
            } // end of match 0

            // sell order
            1 => {
                let mut sell_order = MinOrder { price: t[0], amount: t[1] };

                loop {
                    // 从买单里check
                    let top_buy_order = buy_heap.peek().cloned();

                    // 买单队列为空
                    if top_buy_order.is_none() {
                        sell_heap.push(sell_order);
                        break;
                    }

                    let tbo = top_buy_order.unwrap();

                    // 卖单价格比买单高, 追加卖单队列
                    if sell_order.price > tbo.price {
                        sell_heap.push(sell_order);
                        break;
                    }

                    // 买单价格比卖单高,可以成交
                    if sell_order.price <= tbo.price {
                        // 完全成交
                        if sell_order.amount == tbo.amount {
                            // 消除卖单, 买单不进队列
                            buy_heap.pop();
                            break;
                        }
                        // 买单多
                        else if sell_order.amount < tbo.amount {
                            // 消除部分买单, 卖单不进队列
                            buy_heap.pop();
                            buy_heap.push(MaxOrder { price: tbo.price, amount: tbo.amount - sell_order.amount });
                            break;
                        }
                        // 卖单多
                        else if sell_order.amount > tbo.amount {
                            // 匹配不完的卖单继续和顶部买单匹配
                            sell_order = MinOrder { price: sell_order.price, amount: sell_order.amount - tbo.amount };
                            buy_heap.pop();
                        }
                    }
                } // end of loop
            } // end of match 1
            _ => {}
        } // end of match case
        // println!("sell_heap {:?}", sell_heap.clone());
        // println!("buy_heap {:?}", buy_heap.clone());
    } // end of for loop

    let left_buy_order_count: i32 = buy_heap.iter().map(|x| x.amount).fold(0, |sum, i| (sum + i) % (1_000_000_000 + 7));
    let left_sell_order_count: i32 = sell_heap.iter().map(|x| x.amount).fold(0, |sum, i| (sum + i) % (1_000_000_000 + 7));

    (left_buy_order_count + left_sell_order_count) % (1_000_000_000 + 7)
}

#[cfg(test)]
mod tests {
    use super::{get_number_of_backlog_orders};

    #[test]
    fn test_case_init() {
        // 没有买单，也没有卖单
        let nums: Vec<Vec<i32>> = vec![];
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(0, result);
    }

    #[test]
    fn test_case_0() {
        let nums: Vec<Vec<i32>> = vec![vec![10, 5, 0]];
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(5, result);
    }

    #[test]
    fn test_case_1() {
        let nums: Vec<Vec<i32>> = vec![vec![10, 5, 0], vec![15, 2, 1]];
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(7, result);
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<Vec<i32>> = vec![vec![10, 5, 0], vec![15, 2, 1], vec![25, 1, 1], vec![30, 4, 0]];
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(6, result);
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<Vec<i32>> = vec![vec![7, 1000000000, 1], vec![15, 3, 0], vec![5, 999999995, 0], vec![5, 1, 1]];
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(999999984, result);
    }

    #[test]
    fn test_case_4() {
        let nums: Vec<Vec<i32>> = vec![vec![1,29,1], vec![22,7,1], vec![24,1,0], vec![25,15,1], vec![18,8,1], vec![8,22,0], vec![25,15,1], vec![30,1,1], vec![27,30,0]] ;
        let result: i32 = get_number_of_backlog_orders(nums);
        assert_eq!(22, result);
    }
}