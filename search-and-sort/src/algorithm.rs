pub trait Algorithm<Args, Returns> {
    fn run(&self, args: Args) -> Returns;
}
