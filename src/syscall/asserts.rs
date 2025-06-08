
const_assert!({
    use core::mem::size_of;
    size_of::<Result>() <= size_of::<(u64, u64)>()
});
