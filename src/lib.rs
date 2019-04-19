#[macro_export]
macro_rules! derive_less {

    // READ TEMPLATES

    // (Struct + Enum)
    {
        $(#[$smeta:meta])* $($svis:ident)? struct ... { $(#[$fmeta:meta])? $($fvis:ident)? ... }
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])?                 ... }
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
        $(#[$emeta:meta])* $($evis:ident)? enum   ... { $(#[$vmeta:meta])?                 ... }
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

    // TRANSFORM ITEMS
 
    // Struct (Fields with both meta and pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_fmeta:meta])* $fname:ident : $fty:ty),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? {
            $(#[$fmeta] $(#[$current_fmeta])* $fvis $fname : $fty),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Struct (Fields with only meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (               )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_fmeta:meta])* $fname:ident : $fty:ty),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? {
            $(#[$fmeta] $(#[$current_fmeta])* $fname : $fty),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Struct (Fields with only pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (               ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_fmeta:meta])* $fname:ident : $fty:ty),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? {
            $($(#[$current_fmeta])* $fvis $fname : $fty),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (          ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields with both meta and pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? $((
            $($(#[$current_fmeta:meta])* $fty:ty),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? $((
            $(#[$fmeta] $(#[$current_fmeta])* $fvis $fty),+
        ))?;
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields with only meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (  $fmeta:meta  ) (               )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? $((
            $($(#[$current_fmeta:meta])* $fty:ty),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)+ $((
            $(#[$fmeta] $(#[$current_fmeta])+ $fty),+
        ))?;
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (  $fmeta  ) (         )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Unit struct / Tuple struct (Fields with only pub)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        (               ) (  $fvis:ident  )
        ($($emeta:meta)*) ($($evis:ident)?)
        ($($vmeta:meta)?)

        $(#[$current_smeta:meta])*
        struct $sname:ident $(< $($generic:tt),+ >)? $((
            $($(#[$current_fmeta:meta])* $fty:ty),+ $(,)?
        ))?;

        $($rest:tt)*
    } => {
        $(#[$current_smeta])*
        $(#[$smeta])*
        $($svis)?
        struct $sname $(< $($generic),+ >)? $((
            $($(#[$current_fmeta])* $fvis $fty),+
        ))?;
        derive_less! {
            ($($smeta)*) ($($svis)?)
            (          ) (  $fvis  )
            ($($emeta)*) ($($evis)?)
            ($($vmeta)?)
            $($rest)*
        }
    };

    // Unnamed field / Unit field Enum (Variants with meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (  $vmeta:meta  )

        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ >)+ {
            $($(#[$current_vmeta:meta])* $vname:ident$(($($fty:ty),+))?),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])*
        $(#[$current_emeta])*
        $($evis)?
        enum $ename $(< $($generic),+ >)+ {
            $(#[$vmeta] $(#[$current_vmeta])* $vname$(($($fty),+))?),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            (  $vmeta  )
            $($rest)*
        }
    };

    // Unnamed field / Unit field Enum (Variants without meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (               )

        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_vmeta:meta])* $vname:ident$(($($fty:ty),+))?),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])?
        $(#[$current_emeta])*
        $($evis)?
        enum $ename $(< $($generic),+ >)? {
            $($(#[$current_vmeta])* $vname$(($($fty),+))?),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            (          )
            $($rest)*
        }
    };

    // Named field Enum (Variants with meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (  $vmeta:meta  )

        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_vmeta:meta])* $vname:ident { $($fname:ident : $fty:ty),+ }),+ $(,)*
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])*
        $(#[$current_emeta])*
        $($evis)?
        enum $ename $(< $($generic),+ >)? {
            $(#[$vmeta] $(#[$current_vmeta])* $vname { $($fname : $fty),+ }),+
        }
        derive_less! {
            ($($smeta)*) ($($svis)?)
            ($($fmeta)?) ($($fvis)?)
            ($($emeta)*) ($($evis)?)
            (  $vmeta  )
            $($rest)*
        }
    };

    // Named field Enum (Variants without meta)
    {
        ($($smeta:meta)*) ($($svis:ident)?)
        ($($fmeta:meta)?) ($($fvis:ident)?)
        ($($emeta:meta)*) ($($evis:ident)?)
        (               )

        $(#[$current_emeta:meta])*
        enum $ename:ident $(< $($generic:tt),+ >)? {
            $($(#[$current_vmeta:meta])* $vname:ident { $($fname:ident : $fty:ty),+ }),+ $(,)?
        }

        $($rest:tt)*
    } => {
        $(#[$emeta])?
        $(#[$current_emeta])*
        $($evis)?
        enum $ename $(< $($generic),+ >)? {
            $($(#[$current_vmeta])* $vname { $($fname : $fty),+ }),+
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
