exports.up = async (knex) => {
    await knex.schema.createTable('users', (table) => {
        table.increments('id').primary();
        table.string('solana_address').unique().notNullable();
        table.string('username').unique(); // Optional: Allow usernames
        table.timestamp('created_at').defaultTo(knex.fn.now());
        table.timestamp('updated_at').defaultTo(knex.fn.now());
    });

    await knex.schema.createTable('skill_nfts', (table) => {
        table.string('mint_address').primary(); // Mint address as primary key
        table.integer('owner_id').unsigned().references('id').inTable('users');
        table.string('player_id', 32); // Use VARCHAR to enforce length limit
        table.string('game_name', 64);
        table.integer('rank').unsigned();
        table.string('playstyle', 128);
        table.specificType('achievements', 'text ARRAY'); // Array for achievements
        table.jsonb('social_links'); // JSONB for flexible storage
        table.timestamp('created_at').defaultTo(knex.fn.now());
        table.timestamp('updated_at').defaultTo(knex.fn.now());
        table.bigInteger('price').unsigned();
        table.boolean('is_listed').defaultTo(false);
    });
};

exports.down = async (knex) => {
    await knex.schema.dropTableIfExists('skill_nfts');
    await knex.schema.dropTableIfExists('users');
};
