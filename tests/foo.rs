#[cfg(test)]
mod tests {
    use aurars_derive::Extractor;

    #[derive(Extractor)]
    struct Foo {
        x: i32,
    }
    
    #[test]
    fn it_works() {
        let f = Foo { x: 5 };
        f.x();
    }
}