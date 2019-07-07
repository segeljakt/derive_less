extern crate proc_macro;

// use proc_macro_hack::proc_macro_hack;
use proc_quote::quote;

const TEMPLATE_ID: &'static str = "__";

#[proc_macro]
pub fn derive_less(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::File {
        shebang,
        attrs,
        items,
    } = syn::parse_macro_input!(input as syn::File);

    let (templates, mut items) = partition(items);

    let mut visitor = Visitor::from(templates);

    for ref mut item in items.iter_mut() {
        visitor.visit(item);
    }

    let new = syn::File {
        shebang,
        attrs,
        items,
    };

    proc_macro::TokenStream::from(quote!(#new))
}

struct Visitor {
    template_struct: Option<syn::ItemStruct>,
    template_enum: Option<syn::ItemEnum>,
    template_fn: Option<syn::ItemFn>,
}

fn partition(items: Vec<syn::Item>) -> (Vec<syn::Item>, Vec<syn::Item>) {
    items.into_iter().partition(|item| match item {
        syn::Item::Struct(item) => item.ident == TEMPLATE_ID,
        syn::Item::Enum(item) => item.ident == TEMPLATE_ID,
        syn::Item::Fn(item) => item.ident == TEMPLATE_ID,
        _ => false,
    })
}

impl Visitor {
    fn from(templates: Vec<syn::Item>) -> Self {
        let mut visitor = Self {
            template_struct: None,
            template_enum: None,
            template_fn: None,
        };
        for template in templates {
            match template {
                syn::Item::Struct(template) => visitor.template_struct = Some(template),
                syn::Item::Enum(template) => visitor.template_enum = Some(template),
                syn::Item::Fn(template) => visitor.template_fn = Some(template),
                _ => {}
            }
        }
        visitor
    }

    fn visit(&mut self, item: &mut syn::Item) {
        match item {
            syn::Item::Struct(item) => {
                if let Some(ref mut template) = self.template_struct {
                    item.attrs.extend(template.attrs.clone());
                    if let syn::Visibility::Inherited = item.vis {
                        item.vis = template.vis.clone();
                    }
                    if let Some(ref mut template) = template.fields.iter().next() {
                        for field in item.fields.iter_mut() {
                            field.attrs.extend(template.attrs.clone());
                            if let syn::Visibility::Inherited = field.vis {
                                field.vis = template.vis.clone();
                            }
                        }
                    }
                }
            }
            syn::Item::Enum(item) => {
                if let Some(ref mut template) = self.template_enum {
                    item.attrs.extend(template.attrs.clone());
                    if let syn::Visibility::Inherited = item.vis {
                        item.vis = template.vis.clone();
                    }
                    if let Some(ref mut template) = template.variants.iter().next() {
                        for variant in item.variants.iter_mut() {
                            variant.attrs.extend(template.attrs.clone());
                        }
                        if let Some(ref mut template) = template.fields.iter().next() {
                            for variant in item.variants.iter_mut() {
                                for field in variant.fields.iter_mut() {
                                    field.attrs.extend(template.attrs.clone());
                                }
                            }
                        }
                    }
                }
            }
            syn::Item::Fn(item) => {
                if let Some(ref mut template) = self.template_fn {
                    item.attrs.extend(template.attrs.clone());
                    if let syn::Visibility::Inherited = item.vis {
                        item.vis = template.vis.clone();
                    }
                }
            }
            _ => {}
        }
    }
}

