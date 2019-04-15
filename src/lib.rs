
#[macro_export]
macro_rules! derive_less {

    // PRE-PROCESSING

    // (Struct + Enum)
    {
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])? $($fvis:ident)? ... }
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])?                   ... }
        $($rest:tt)*
    } => {
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // (Enum + Struct)
    {
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])?                 ... }
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])? $($fvis:ident)? ... }
        $($rest:tt)*
    } => {
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Struct
    {
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])? $($fvis:ident)? ... }
        $($rest:tt)*
    } => {
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            (          ) (         )
            (          )
            $($rest)*
        }
    };

    // Enum
    {
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])?                   ... }
        $($rest:tt)*
    } => {
        derive_less! {
            (          ) (         )
            (          ) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // TRANSFORMATION

    // Tuple struct (With both meta and pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $name:ident $(< $($generic:tt),* >)* (
            $( $(#[$current_fmeta:meta])* $field:ty),* $(,)*
        );

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $name $(< $($generic),* >)* (
            $(
                #[$fmeta]
                $(#[$current_fmeta:meta])*
                $fvis
                $field
            ),*
        );
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Tuple struct (With only meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (               )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $name:ident $(< $($generic:tt),* >)* (
            $( $(#[$current_fmeta:meta])* $field:ty),* $(,)*
        );

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $name $(< $($generic),* >)* (
            $(
                #[$fmeta]
                $(#[$current_fmeta:meta])*
                $field
            ),*
        );
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Tuple struct (With only pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (               ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $name:ident $(< $($generic:tt),* >)* (
            $( $(#[$current_fmeta:meta])* $field:ty),* $(,)*
        );

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $name $(< $($generic),* >)* (
            $(
                $(#[$current_fmeta:meta])*
                $fvis
                $field
            ),*
        );
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (          ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Enum (With meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (  $vmeta:meta  )

        $(#[$current_emeta:meta])*
        enum $name:ident $(< $($generic:tt),* >)* {
            $( $(#[$current_vmeta:meta])* $variant:ident($($field:ty),*)),* $(,)*
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])*
        $(#[$current_emeta])*
        $($evis)?
        enum $name $(< $($generic),* >)* {
            $(
                #[$vmeta]
                $(#[$current_vmeta])*
                $variant($($field),*)
            ),*
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            (  $vmeta  )
            $($rest)*
        }
    };

    // Enum (Without meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (               )

        $(#[$current_emeta:meta])*
        enum $name:ident $(< $($generic:tt),* $(,)* >)* {
            $( $(#[$current_vmeta:meta])* $variant:ident($($field:ty),*)),* $(,)*
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])?
        $(#[$current_emeta])*
        $($evis)?
        enum $name $(< $($generic),* >)* {
            $(
                $(#[$current_vmeta])*
                $variant($($field),*)
            ),*
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            (          )
            $($rest)*
        }
    };

    // Exhausted
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)
        $($rest:tt)*
    } => {
        $($rest)*
    }
}
