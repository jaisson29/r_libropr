pub fn get_by_email(state: &AppState, email: &str) -> AppResult<Persona> {
  let persona_repository = PersonaRepositoryMySQL::new(state.db.clone());
}
