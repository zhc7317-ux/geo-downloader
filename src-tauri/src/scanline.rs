//! 大图多边形裁剪共用的扫描线交点计算。
//!
//! 像素坐标本身仍可使用 i32；乘法的中间值必须使用 i64，避免大图出现透明横带。

/// 计算一条非水平边与指定扫描行相交时的横坐标。
///
/// 输入：xi、yi 和 xj、yj 是边的两个端点像素坐标；row 是当前行号。
/// 调用条件：边跨过当前行，即 (yi > row) != (yj > row)，确保分母不为零。
/// 输出：交点横坐标（i64，单位为像素），使用原算法相同的向零取整规则。
/// 注意：必须在减法、乘法之前转换类型；乘法后才转 i64 无法修复溢出。
pub(crate) fn intersection_x(xi: i32, yi: i32, xj: i32, yj: i32, row: i32) -> i64 {
    let (xi, yi, xj, yj, row) = (
        i64::from(xi), i64::from(yi), i64::from(xj), i64::from(yj), i64::from(row),
    );
    // 例如 60000 * 45000 = 27 亿，超过 i32 上限，但可以由 i64 正确保存。
    (xj - xi) * (row - yi) / (yj - yi) + xi
}

#[cfg(test)]
mod tests {
    use super::intersection_x;

    /// 输入：中间乘积为 27 亿的上升边和下降边；输出：正确的交点断言通过。
    #[test]
    fn large_edges_do_not_overflow() {
        assert_eq!(intersection_x(0, 0, 60000, 60000, 45000), 45000);
        assert_eq!(intersection_x(60000, 0, 0, 60000, 45000), 15000);
        assert_eq!(intersection_x(60000, 60000, 0, 0, 45000), 45000);
    }

    /// 输入：小图中的斜边、竖边及端点；输出：保持原有整数取整和边界结果。
    #[test]
    fn small_edges_keep_existing_rounding() {
        assert_eq!(intersection_x(0, 0, 10, 3, 1), 3);
        assert_eq!(intersection_x(10, 0, 0, 3, 1), 7);
        assert_eq!(intersection_x(7, 0, 7, 100, 50), 7);
        assert_eq!(intersection_x(10, 20, 30, 40, 20), 10);
    }

    /// 输入：按实际预览估计的 88576×64512 大图边界；输出：原白带区间中心均在选区内。
    /// 只计算交点，不分配整幅大图，因此测试不需要占用数十 GB 内存。
    #[test]
    fn large_polygon_has_no_horizontal_gaps() {
        let ring = [(77395, 64485), (88419, 31400), (15557, -1), (-123, 33409)];
        for row in 26600..40000 {
            let mut crossings = Vec::new();
            for index in 0..ring.len() {
                let (xi, yi) = ring[index];
                let (xj, yj) = ring[(index + ring.len() - 1) % ring.len()];
                if (yi > row) != (yj > row) {
                    crossings.push(intersection_x(xi, yi, xj, yj, row));
                }
            }
            crossings.sort_unstable();
            assert_eq!(crossings.len(), 2);
            assert!(crossings[0] <= 44288 && 44288 < crossings[1], "row={row}");
        }
    }
}
