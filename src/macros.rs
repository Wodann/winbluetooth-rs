macro_rules! _WSAIORW {
    ($x:expr, $y:expr) => {
        winapi::shared::ws2def::IOC_INOUT | $x | $y
    };
}
macro_rules! BTH_CTL {
    ($id:expr) => {
        CTL_CODE!(FILE_DEVICE_BLUETOOTH, $id, METHOD_BUFFERED, FILE_ANY_ACCESS)
    };
}
macro_rules! BTH_KERNEL_CTL {
    ($id:expr) => {
        CTL_CODE!(FILE_DEVICE_BLUETOOTH, $id, METHOD_NEITHER, FILE_ANY_ACCESS)
    };
}
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    };
}
macro_rules! DEFINE_BLUETOOTH_UUID128 {
    ($name:ident, $shortId:expr) => {
        DEFINE_GUID! {$name,
        $shortId as u32, 0x0000, 0x1000, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB}
    };
}
macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: winapi::shared::guiddef::GUID = winapi::shared::guiddef::GUID {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    };
}
macro_rules! ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = u32;
        $(pub const $variant: $name = $value;)+
    };
    {enum $name:ident { $variant:ident = $value:expr, $($rest:tt)* }} => {
        pub type $name = u32;
        pub const $variant: $name = $value;
        ENUM!{@gen $name $variant, $($rest)*}
    };
    {enum $name:ident { $variant:ident, $($rest:tt)* }} => {
        ENUM!{enum $name { $variant = 0, $($rest)* }}
    };
    {@gen $name:ident $base:ident,} => {};
    {@gen $name:ident $base:ident, $variant:ident = $value:expr, $($rest:tt)*} => {
        pub const $variant: $name = $value;
        ENUM!{@gen $name $variant, $($rest)*}
    };
    {@gen $name:ident $base:ident, $variant:ident, $($rest:tt)*} => {
        pub const $variant: $name = $base + 1u32;
        ENUM!{@gen $name $variant, $($rest)*}
    };
}
macro_rules! FN {
    (stdcall $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($t,)*) -> $ret>;
    );
    (stdcall $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "system" fn($($p: $t,)*) -> $ret>;
    );
    (cdecl $func:ident($($t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($t,)*) -> $ret>;
    );
    (cdecl $func:ident($($p:ident: $t:ty,)*) -> $ret:ty) => (
        pub type $func = Option<unsafe extern "C" fn($($p: $t,)*) -> $ret>;
    );
}
macro_rules! STRUCT {
    (#[debug] $($rest:tt)*) => (
        STRUCT!{#[cfg_attr(feature = "impl-debug", derive(Debug))] $($rest)*}
    );
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { std::mem::zeroed() } }
        }
    );
}
macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { std::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_arch = "x86")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { std::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}
