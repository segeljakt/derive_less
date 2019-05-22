#[macro_export]
macro_rules! derive_less {

    // READ TEMPLATES

    // (Struct + Enum)
    {
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])* $($fvis:ident)? ... }
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])*                 ... }
        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // (Enum + Struct)
    {
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])*                 ... }
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])* $($fvis:ident)? ... }
        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Struct
    {
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])* $($fvis:ident)? ... }
        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            (          ) (         )
            (          )
            $($rest)*
        }
    };

    // Enum
    {
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])*                 ... }
        $($rest:tt)*
    } => {
        $crate::derive_less! {
            (          ) (         )
            (          ) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // TRANSFORM ITEMS
 
    // Struct (Begin)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        $(#[$current_smeta:meta])*
        struct

        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)

            ($($fmeta)*)
            $(#[$current_smeta])*
            struct

            $($rest)*
        }
    };

    // Struct (Fields with meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ($first:meta $($apply:meta)*)
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? {
            $(
                $(#[$current_fmeta:meta])*
                $fname:ident : $fty:ty
            ),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)

            ($($apply)*)
            $(#[$current_smeta])*
            struct $sname $(< $($generic),+ >)? {
                $(
                    $(#[$current_fmeta])*
                    #[$first]
                    $fname : $fty
                ),+
            }

            $($rest)*
        }
    };

    // Struct (Fields with pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ()
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? {
            $(
                $(#[$current_fmeta:meta])*
                $fname:ident : $fty:ty
            ),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? {
            $(
                $(#[$current_fmeta])*
                $fvis $fname : $fty
            ),+
        }

        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Struct (Fields without pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) (               )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ()
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? {
            $(
                $(#[$current_fmeta:meta])*
                $fname:ident : $fty:ty
            ),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? {
            $(
                $(#[$current_fmeta])*
                $fname : $fty
            ),+
        }

        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields with meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ($first:meta $($apply:meta)*)
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? $((
            $(
                $(#[$current_fmeta:meta])*
                $fty:ty
            ),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)

            ($($apply)*)
            $(#[$current_smeta])*
            struct $sname $(< $($generic),+ >)? $((
                $(
                    $(#[$current_fmeta])*
                    #[$first]
                    $fty
                ),+
            ))?;

            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields with pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ()
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? $((
            $(
                $(#[$current_fmeta:meta])*
                $fty:ty
            ),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? $((
            $(
                $(#[$current_fmeta])*
                $fvis $fty
            ),+
        ))?;

        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields without pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) (               )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ()
        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ $(,)? >)? $((
            $(
                $(#[$current_fmeta:meta])*
                $fty:ty
            ),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? $((
            $(
                $(#[$current_fmeta])*
                $fty
            ),+
        ))?;

        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Enum (Begin)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        $(#[$current_emeta:meta])*
        enum

        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)

            ($($vmeta)*)
            $(#[$current_emeta])*
            enum

            $($rest)*
        }
    };

    // Enum (Variants with meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ($first:meta $($apply:meta)*)
        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ $(,)? >)? {
            $(
                $(#[$current_vmeta:meta])*
                $vname:ident
                $(( $($vty:ty),+ $(,)? ))?
                $({ $($fname:ident : $fty:ty),+ $(,)? })?
            ),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)

            ($($apply)*)
            $(#[$current_emeta])*
            enum $ename $(< $($generic),+ >)? {
                $(
                    $(#[$current_vmeta])*
                    #[$first]
                    $vname
                    $(( $($vty),+ ))?
                    $({ $($fname : $fty),+ })?
                ),+
            }

            $($rest)*
        }
    };

    // Enum (Done)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)

        ()
        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ $(,)? >)? {
            $(
                $(#[$current_vmeta:meta])*
                $vname:ident
                $(( $($vty:ty),+ $(,)? ))?
                $({ $($fname:ident : $fty:ty),+ $(,)? })?
            ),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])*
        $(#[$current_emeta])*
        $($evis)?
        enum $ename $(< $($generic),+ >)? {
            $(
                $(#[$current_vmeta])*
                $vname
                $(( $($vty),+ ))?
                $({ $($fname : $fty),+ })?
            ),+
        }

        $crate::derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)*) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            ($($vmeta)*)
            $($rest)*
        }
    };

    // Exhausted
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)*) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)*)
//         $($rest:tt)*
    } => {
//         $($rest)*
    }
}
