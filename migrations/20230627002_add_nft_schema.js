exports.up = async (knex) => {
    // Add additional fields to the skill_nfts table
    await knex.schema.alterTable('skill_nfts', (table) => {
        table.string('image_url');  // Add a field to store the image URL of the NFT
        table.string('metadata_uri'); // Add a field to store the metadata URI of the NFT
    });
};

exports.down = async (knex) => {
    // Revert the changes made in the up function
    await knex.schema.alterTable('skill_nfts', (table) => {
        table.dropColumn('image_url');
        table.dropColumn('metadata_uri');
    });
};
