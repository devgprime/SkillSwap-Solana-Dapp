exports.up = function(knex) {
  return knex.schema.createTable('users', function(table) {
    table.increments('id').primary();
    table.string('username').notNullable();
    table.string('email').notNullable().unique();
    table.string('passwordHash').notNullable();
    table.text('bio');
    table.json('skills');
    table.json('achievements');
    table.json('socialLinks');
    table.string('profilePicture');
    table.timestamps(true, true);
  });
};

exports.down = function(knex) {
  return knex.schema.dropTableIfExists('users');
};

