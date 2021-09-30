///
/// http://llever.com/exercism-rust-zh/leap/README.zh.html
///
/// 目前使用的格里高利历闰年规则如下[1]：
/// 公元年分非4的倍数，为平年。
/// 公元年分为4的倍数但非100的倍数，为闰年。
/// 公元年分为100的倍数但非400的倍数，为平年。
/// 公元年分为400的倍数为闰年。
///
///
pub fn is_leap_year(year: i32) -> bool {
    if year % 4 != 0 {
        // 公元年分非4的倍数，为平年。
        return false;
    }

    if year % 100 != 0 {
        // 公元年分为4的倍数但非100的倍数，为闰年。
        return true;
    }

    if year % 400 != 0 {
        // 公元年分为100的倍数但非400的倍数，为平年。
        return false;
    }

    true
}
