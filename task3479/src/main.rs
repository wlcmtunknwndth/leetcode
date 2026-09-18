struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut st = SegmentTree {
            n,
            tree: vec![0; 4 * n],
        };

        st.build(arr, 1, 0, n - 1);

        st
    }

    fn build(&mut self, arr: &[i32], node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = arr[left];
            return;
        }

        let mid = (left + right) / 2;

        self.build(arr, node * 2, left, mid);
        self.build(arr, node * 2 + 1, mid + 1, right);

        self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
    }

    fn find_leftmost(&self, x: i32, node: usize, left: usize, right: usize) -> Option<usize> {
        if self.tree[node] < x {
            return None;
        }

        if left == right {
            return Some(left);
        }

        let mid = (left + right) / 2;

        if let Some(index) = self.find_leftmost(x, node * 2, left, mid) {
            return Some(index);
        }

        self.find_leftmost(x, node * 2 + 1, mid + 1, right)
    }

    fn update(&mut self, index: usize, value: i32, node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = value;
            return;
        }

        let mid = (left + right) / 2;

        if index <= mid {
            self.update(index, value, node * 2, left, mid);
        } else {
            self.update(index, value, node * 2 + 1, mid + 1, right);
        }

        self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
    }
}

struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut st = SegmentTree::new(&baskets);
        let mut ans = 0i32;
        for fruit in fruits {
            if let Some(index) = st.find_leftmost(fruit, 1, 0, st.n - 1) {
                st.update(index, 0, 1, 0, st.n - 1);
            } else {
                ans += 1;
            }
        }

        return ans;
    }
}

fn main() {
    // #1:
    //  Input: fruits = [4,2,5], baskets = [3,5,4]
    //  Output: 1
    let (mut fruits, mut baskets) = (vec![4, 2, 5], vec![3, 5, 4]);
    let mut ans = 1i32;
    assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), ans);

    // #2:
    // Input: fruits = [3,6,1], baskets = [6,4,7]
    // Output: 0
    (fruits, baskets) = (vec![3, 6, 1], vec![6, 4, 7]);
    ans = 0;
    assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), ans);
}
