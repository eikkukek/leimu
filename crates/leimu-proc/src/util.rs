pub fn find_attr<'a>(
    input: &'a syn::DeriveInput,
    ident: &str,
) -> Option<&'a syn::Attribute>
{
    input.attrs
        .iter()
        .find(|attr|
            attr.path().segments
                .iter().last()
                .map(|segment| segment.ident == ident)
                .unwrap_or(false)
        )
}
