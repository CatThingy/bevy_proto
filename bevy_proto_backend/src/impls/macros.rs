/// Registers the given types along with their respective input types and [`ReflectSchematic`].
#[macro_export]
#[doc(hidden)]
macro_rules! register_schematic {
    ($app: ident, $($ty: ty),* $(,)?) => {{
        $(
            // Sanity check: ensure the type is actually registered
            // before actually registering the `ReflectSchematic` type data
            $app.register_type::<$ty>()
                .register_type::<<$ty as $crate::schematics::Schematic>::Input>()
                .register_type_data::<$ty, $crate::schematics::ReflectSchematic>();

        )*
    }};
}

pub(super) use register_schematic;

/// Implements `From` going from `$real` to `$mock`.
///
/// The `$body` should be a closure that takes in a value of type `Input`
/// and maps to `Self`.
#[macro_export]
#[doc(hidden)]
macro_rules! from {
    ($real: ty, $mock: ty, $body: expr) => {
        const _: () = {
            type Input = $mock;

            impl From<Input> for $real {
                fn from(value: Input) -> Self {
                    $body(value)
                }
            }
        };
    };
}

/// Implements `From` going from `$real` to `$mock` and vice-versa.
///
/// The `$body` should be a closure that takes in a value of type `Input`
/// and maps to `Self`.
#[macro_export]
#[doc(hidden)]
macro_rules! from_to {
    ($real: ty, $mock: ty, $body: expr) => {
        const _: () = {
            type Input = $real;

            impl From<Input> for $mock {
                fn from(value: Input) -> Self {
                    $body(value)
                }
            }
        };

        const _: () = {
            type Input = $mock;

            impl From<Input> for $real {
                fn from(value: Input) -> Self {
                    $body(value)
                }
            }
        };
    };
}

pub(super) use from_to;

#[macro_export]
#[doc(hidden)]
macro_rules! from_to_input {
    ($real: ty, $mock: ty, $body: expr) => {
        const _: () = {
            type Input = $real;

            impl $crate::schematics::FromSchematicInput<Input> for $mock {
                fn from_input(
                    input: Input,
                    context: &mut $crate::schematics::SchematicContext,
                ) -> Self {
                    $body(input, context)
                }
            }
        };

        const _: () = {
            type Input = $mock;

            impl $crate::schematics::FromSchematicInput<Input> for $real {
                fn from_input(
                    input: Input,
                    context: &mut $crate::schematics::SchematicContext,
                ) -> Self {
                    $body(input, context)
                }
            }
        };
    };
}

pub(super) use from_to_input;

/// Implements `From` going from `$real` to `$mock` and vice-versa,
/// as well as `Default` for `$mock` using `$real`.
///
/// The `$body` should be a closure that takes in a value of type `Input`
/// and maps to `Self`.
#[macro_export]
#[doc(hidden)]
macro_rules! from_to_default {
    ($real: ty, $mock: ty, $body: expr) => {
        $crate::from_to!($real, $mock, $body);

        const _: () = {
            impl Default for $mock {
                fn default() -> Self {
                    <$real as Default>::default().into()
                }
            }
        };
    };
}

pub(super) use from_to_default;
