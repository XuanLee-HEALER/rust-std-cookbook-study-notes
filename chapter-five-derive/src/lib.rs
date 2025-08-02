use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Lit};

/// 在 proc_macro crate中，只有注解为 proc_macro_derive 的函数可以是 pub 的
/// 这个属性需要指定 derive 的名字，和它允许什么属性，因为这个方法直接挂在（hook）编译器上，所以它的输入输出都是 TokenStream
/// 过程宏不改变代码，只是分析它然后增加一些代码
///
/// cargo-expand工具可以展示编译器会如何扩展你的过程宏
#[proc_macro_derive(HelloWorld, attributes(hello_world_name))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_str(&s).expect("Failed to parse the source into an AST");
    impl_hello_world(&ast)
}

// ident表示 identifier ，告诉我们结构体或者枚举的名称
fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
    let identifier = &ast.ident;

    let hello_world_name = get_name_attribute(ast).unwrap_or_else(|| identifier.to_string());

    quote! {
        impl HelloWorld for #identifier {
            fn hello_world() {
                println!(
                    "The struct or enum {} says: \"Hello world from {}!\"",
                    stringify!(#identifier),
                    #hello_world_name
                );
            }
        }
    }
    .into()
}

fn get_name_attribute(ast: &syn::DeriveInput) -> Option<String> {
    const ATTR_NAME: &str = "hello_world_name";
    if let Some(attr) = ast.attrs.iter().find(|a| {
        a.meta
            .require_name_value()
            .map(|a| a.path.is_ident(ATTR_NAME))
            .unwrap_or(true)
    }) {
        if let Ok(ref mnv) = attr.meta.require_name_value() {
            if let Expr::Lit(ref value) = mnv.value {
                if let Lit::Str(ref value_as_str) = value.lit {
                    Some(value_as_str.value())
                } else {
                    // 和普通程序不同，过程宏 panic! 是在编译期调用，所以经常使用没关系
                    panic!(
                        "Expected a string as the value of {}, found others instead",
                        ATTR_NAME
                    )
                }
            } else {
                panic!(
                    "Expected a string as the value of {}, found others instead",
                    ATTR_NAME
                )
            }
        } else {
            panic!(
                "Expected an attribute in the form #[{} = \"Some value\"]",
                ATTR_NAME
            )
        }
    } else {
        None
    }
}
