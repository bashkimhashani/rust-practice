#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let cols = matrix[0].len();
        let mut dp = vec![0; cols + 1];
        let mut largest_side = 0;

        for row in &matrix {
            let mut diagonal = 0;

            for (col, &cell) in row.iter().enumerate() {
                let j = col + 1;
                let top = dp[j];

                if cell == '1' {
                    dp[j] = 1 + dp[j].min(dp[j - 1]).min(diagonal);
                    largest_side = largest_side.max(dp[j]);
                } else {
                    dp[j] = 0;
                }

                diagonal = top;
            }
        }

        (largest_side * largest_side) as i32
    }
}