#[macro_export]
macro_rules! capnp_init(
  ($builder: ident, $init: ident, $([$($subinit: tt)*],)*) => {{
    let mut _builder = $builder.borrow().$init();
    $(
      capnp_init!(_builder, $($subinit)*);
    )*
  }};

  ($builder: ident, $setter: ident, $val: expr) => {{
    $builder.$setter($val);
  }};

  ($builder: ident, $init: ident, $len: expr, $([$([$($subinit: tt)*],)*],)*) => {{
    let mut _builder = $builder.borrow().$init($len);
    let _i = 0;
    $(
      {
        let mut _elem = _builder.borrow().get(_i);
        $(
          capnp_init!(_elem, $($subinit)*);
        )*
      }
      let _i = _i + 1;
    )*
  }};
);

#[macro_export]
macro_rules! capnp_new(
  ($ty:ty, $([$($tt:tt)*],)*) => {{
    let mut message = MallocMessageBuilder::new_default();
    {
      let mut message = message.init_root::<$ty>();
      $(
        capnp_init!(message, $($tt)*);
      )*
    }
    message
  }};
);