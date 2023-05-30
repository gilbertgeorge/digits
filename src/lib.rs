mod utils;

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Digits {
    digits: VecDeque<u32>,
    target: u32,
}

#[wasm_bindgen]
impl Digits {
    pub fn new(target: u32, digits: Vec<u32>) -> Digits {
        utils::set_panic_hook();
        Digits {
            digits: digits.into_iter().collect(),
            target,
        }
    }

    pub fn solve(&mut self) -> JsValue {
        let results = brute_force_solutions(self.target, self.digits.clone(), &[]);
        serde_wasm_bindgen::to_value(&results).unwrap()
    }
}

fn brute_force_solutions(target: u32, mut numbers: VecDeque<u32>, path: &[String]) -> Vec<Vec<String>> {
    numbers.make_contiguous().sort_by(|a, b| b.cmp(a));
    let combos = nc2(&numbers);
    let mut valid_paths = Vec::new();

    for (a, b, rest) in combos {
        let (prod, valid_prod, prod_path) = (
            a * b,
            a > 1 && b > 1,
            push_immutable(path, format!("{a} * {b}")),
        );
        let (add, valid_add, add_path) = (a + b, true, push_immutable(path, format!("{a} + {b}")));
        let (div, div_exact, div_path) = (
            a / b,
            a % b == 0 && b != 1,
            push_immutable(path, format!("{a} / {b}")),
        );
        let (sub, sub_positive, sub_path) = (
            a - b,
            a - b > 0 && a - b != b,
            push_immutable(path, format!("{a} - {b}")),
        );

        if valid_prod && prod == target {
            valid_paths.push(prod_path.clone());
        } else if add == target {
            valid_paths.push(add_path.clone());
        } else if div_exact && div == target {
            valid_paths.push(div_path.clone());
        } else if sub_positive && sub == target {
            valid_paths.push(sub_path.clone());
        }

        if (valid_prod && prod == target)
            || (valid_add && add == target)
            || (div_exact && div == target)
            || (sub_positive && sub == target)
        {
            return valid_paths;
        }

        if valid_prod {
            let mut prod_nums = rest.clone();
            prod_nums.push_front(prod);
            valid_paths.extend(brute_force_solutions(target, prod_nums, &prod_path));
        }

        if valid_add {
            let mut add_nums = rest.clone();
            add_nums.push_front(add);
            valid_paths.extend(brute_force_solutions(target, add_nums, &add_path));
        }

        if div_exact {
            let mut div_nums = rest.clone();
            div_nums.push_front(div);
            valid_paths.extend(brute_force_solutions(target, div_nums, &div_path));
        }

        if sub_positive {
            let mut sub_nums = rest.clone();
            sub_nums.push_front(sub);
            valid_paths.extend(brute_force_solutions(target, sub_nums, &sub_path));
        }
    }

    valid_paths
}

fn nc2(numbers: &VecDeque<u32>) -> VecDeque<(u32, u32, VecDeque<u32>)> {
    let mut res = VecDeque::new();
    let mut rest = numbers.clone();

    while let Some(n) = rest.pop_front() {
        for m in rest.clone() {
            let others = numbers
                .iter()
                .filter(|i| *i.to_owned() != n && *i.to_owned() != m)
                .map(std::borrow::ToOwned::to_owned)
                .collect::<VecDeque<u32>>();
            res.push_back((n, m, others));
        }
    }

    res
}

fn push_immutable(elements: &[String], insert: String) -> Vec<String> {
    let mut new = elements.to_owned();
    new.push(insert);
    new
}
