
diesel_down:
	diesel migration run
diesel_up:
	diesel migration redo
diesel_generate_migration:
	diesel migration generate --diff-schema create_conditions
	diesel migration generate --diff-schema create_conditions_default
diesel_generate_schema:
	diesel migration run