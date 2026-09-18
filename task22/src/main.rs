pub fn dfs(n: i32, o_cnt: i32, c_cnt: i32, path: &mut String, res: &mut Vec<String>) {
    if o_cnt == n && c_cnt == n {
        res.push(path.clone());
        return;
    }

    if o_cnt < n {
        path.push('(');
        dfs(n, o_cnt + 1, c_cnt, path, res);
        path.pop();
    }

    if c_cnt < o_cnt {
        path.push(')');
        dfs(n, o_cnt, c_cnt + 1, path, res);
        path.pop();
    }

    return;
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut path: String = String::new();

    dfs(n, 0, 0, &mut path, &mut res);

    return res;
}

fn main() {
    assert_eq!(
        generate_parenthesis(3),
        ["((()))", "(()())", "(())()", "()(())", "()()()"]
    );

    assert_eq!(generate_parenthesis(1), ["()"]);
}
