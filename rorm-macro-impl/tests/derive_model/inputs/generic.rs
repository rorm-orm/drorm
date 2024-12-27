#[derive(Model)]
#[rorm(experimental_generics, experimental_unregistered)]
pub struct Foo<X: 'static + FieldType> {
    #[rorm(id)]
    pub id: i64,

    pub x: X,
}
