#[repr(u16)]
pub enum ConditionFlags {
    Positive = 1 << 0, // 1
    Zero = 1 << 1,     // 2
    Negative = 1 << 2, // 4
}
