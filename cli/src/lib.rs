mod main_loop;

pub fn present_interface() {
    main_loop::execute_main_loop();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
