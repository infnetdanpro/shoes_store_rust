use minijinja::Environment;

#[derive(Debug)]
pub struct AppState {
    pub tpl_env: Environment<'static>,
}
